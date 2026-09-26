use std::io::{self, IsTerminal, Write as _};

use reservist_content::{ContentError, frozen::validate_scenario_with_catalog};
use reservist_core::api::{
    Command, CommandAction, FrozenScenario, RequestTiming, Session, View, views::Projection,
};

#[derive(clap::Args)]
pub(crate) struct PlayArgs {
    #[command(flatten)]
    source: crate::ScenarioArgs,
    #[arg(long, default_value = "MEASURED_FIRMING")]
    package: String,
}

const COMMANDS: &str = "Commands: inspect <number> | ask markets [accelerated] | advance | book | verbs | fomc | propose <package> | operations | statement | wire | review | quit";
const CAMPAIGN_COMMANDS: &str = "Commands: inspect <number> | ask markets [accelerated] | advance | book | calendar | folder | verbs | fomc | propose <package> | operations | statement | wire | review | scorecard | revise-program <id> | dispose-review <id> <version> <accept|respond|revise> [response-record] | supplement-review <id> <version> <supplemental-id> | quit";

fn failure(error: impl ToString) -> ContentError {
    ContentError::new("session", error.to_string())
}

fn command_banner(session: &Session) -> &'static str {
    if session
        .available_verbs()
        .is_ok_and(|verbs| verbs.iter().any(|verb| verb == "dispose_review"))
    {
        CAMPAIGN_COMMANDS
    } else {
        COMMANDS
    }
}

enum Input {
    Quit,
    Output(String),
}

fn submit(session: &mut Session, action: CommandAction) -> reservist_core::api::Receipt {
    let command_id = session.next_command_id();
    session.submit(Command {
        idempotency_key: command_id.clone(),
        command_id,
        action,
    })
}

fn receipt_text(
    receipt: reservist_core::api::Receipt,
    missing: &str,
) -> Result<String, ContentError> {
    if !receipt.accepted {
        return Ok(receipt.reason.unwrap_or_else(|| "command rejected".into()));
    }
    receipt
        .projection
        .map(|projection| projection.text().into())
        .or(receipt.message)
        .ok_or_else(|| failure(missing))
}

fn command_argument<'a>(command: &'a str, prefix: &str) -> Option<&'a str> {
    command
        .strip_prefix(prefix)
        .map(str::trim)
        .filter(|value| !value.is_empty())
}

fn perform(session: &mut Session, command: &str) -> Result<Input, ContentError> {
    let view = match command {
        "book" => Some(View::Book),
        "fomc" => Some(View::Fomc),
        "operations" => Some(View::Operations),
        "statement" => Some(View::Statement),
        "wire" => Some(View::Wire),
        "review" => Some(View::Review),
        "calendar" => Some(View::Calendar),
        "scorecard" => Some(View::Scorecard),
        _ => None,
    };
    if let Some(view) = view {
        return Ok(Input::Output(
            session.view(view).map_err(failure)?.text().into(),
        ));
    }
    let output = match command {
        "quit" => return Ok(Input::Quit),
        "verbs" => format!(
            "Available: {}",
            session.available_verbs().map_err(failure)?.join(" | ")
        ),
        "advance" => receipt_text(
            submit(session, CommandAction::Advance),
            "advance has no result message",
        )?,
        "ask markets" | "ask markets accelerated" => {
            let mode = if command.ends_with("accelerated") {
                RequestTiming::Accelerated
            } else {
                RequestTiming::Normal
            };
            let receipt = submit(session, CommandAction::RequestFollowUp { mode });
            if receipt.accepted {
                receipt_text(receipt, "request has no projection")?
            } else {
                format!(
                    "request rejected: {}",
                    receipt.reason.unwrap_or_else(|| "command rejected".into())
                )
            }
        }
        "folder" => receipt_text(
            submit(
                session,
                CommandAction::OpenFolder {
                    folder_id: "folder.policy_cycle".into(),
                },
            ),
            "folder has no folder projection",
        )?,
        "handoff" => receipt_text(
            submit(session, CommandAction::HandOff),
            "handoff has no result",
        )?,
        "close-folder" => receipt_text(
            submit(session, CommandAction::CloseWithoutHandoff),
            "close-folder has no result",
        )?,
        command if command.starts_with("revise-program ") => receipt_text(
            submit(
                session,
                CommandAction::ReviseChairmanshipProgram {
                    program_id: command_argument(command, "revise-program ")
                        .unwrap_or_default()
                        .into(),
                    revision_id: format!("revision.cli.{}", session.next_command_id()),
                },
            ),
            "program revision has no result",
        )?,
        command if command.starts_with("dispose-review ") => {
            let mut arguments = command["dispose-review ".len()..].split_whitespace();
            match (
                arguments.next(),
                arguments.next().and_then(|value| value.parse::<u32>().ok()),
                arguments.next(),
                arguments.next(),
                arguments.next(),
            ) {
                (Some(review_id), Some(review_version), Some("accept"), None, None) => receipt_text(
                    submit(session, CommandAction::DisposeReview {
                        review_id: review_id.into(),
                        review_version,
                        disposition: reservist_core::campaign::ReviewDisposition::Accept,
                        response_record_id: None,
                    }),
                    "review disposition has no result",
                )?,
                (Some(review_id), Some(review_version), Some("respond"), Some(response), None) => receipt_text(
                    submit(session, CommandAction::DisposeReview {
                        review_id: review_id.into(),
                        review_version,
                        disposition: reservist_core::campaign::ReviewDisposition::AcceptWithChairResponse,
                        response_record_id: Some(response.into()),
                    }),
                    "review disposition has no result",
                )?,
                (Some(review_id), Some(review_version), Some("revise"), None, None) => receipt_text(
                    submit(session, CommandAction::DisposeReview {
                        review_id: review_id.into(),
                        review_version,
                        disposition: reservist_core::campaign::ReviewDisposition::RequestRevision,
                        response_record_id: None,
                    }),
                    "review disposition has no result",
                )?,
                _ => "invalid dispose-review command: expected dispose-review <id> <version> <accept|respond|revise> [response-record]".into(),
            }
        }
        command if command.starts_with("supplement-review ") => {
            let mut arguments = command["supplement-review ".len()..].split_whitespace();
            match (
                arguments.next(),
                arguments.next().and_then(|value| value.parse::<u32>().ok()),
                arguments.next(),
                arguments.next(),
            ) {
                (Some(review_id), Some(review_version), Some(supplemental_review_id), None) => {
                    receipt_text(
                        submit(session, CommandAction::CommissionSupplementalReview {
                            review_id: review_id.into(),
                            review_version,
                            supplemental_review_id: supplemental_review_id.into(),
                        }),
                        "supplemental review has no result",
                    )?
                }
                _ => "invalid supplement-review command: expected supplement-review <id> <version> <supplemental-id>".into(),
            }
        }
        command if command.starts_with("propose ") => {
            let package_id = command_argument(command, "propose ").unwrap_or_default();
            let receipt = submit(
                session,
                CommandAction::Propose {
                    package_id: package_id.into(),
                },
            );
            if !receipt.accepted && receipt.category.as_deref() == Some("unknown_package") {
                format!("unknown package: {package_id}")
            } else {
                receipt_text(receipt, "proposal has no room projection")?
            }
        }
        command if command.starts_with("preview ") => {
            let card = session
                .preview_option(command_argument(command, "preview ").unwrap_or_default())
                .map_err(failure)?;
            format!(
                "{}\nEXACT\n{}\nASSESSMENT\n{}",
                card.title,
                card.exact.join("\n"),
                card.assessment.join("\n")
            )
        }
        command if command.starts_with("speak ") => receipt_text(
            submit(
                session,
                CommandAction::CommitSpokenLine {
                    option_id: command_argument(command, "speak ")
                        .unwrap_or_default()
                        .into(),
                },
            ),
            "speaking commit has no folder projection",
        )?,
        command if command.starts_with("pencil ") => receipt_text(
            submit(
                session,
                CommandAction::Pencil {
                    option_id: command_argument(command, "pencil ")
                        .unwrap_or_default()
                        .into(),
                },
            ),
            "pencil has no folder projection",
        )?,
        command if command.starts_with("folder ") => receipt_text(
            submit(
                session,
                CommandAction::OpenFolder {
                    folder_id: command_argument(command, "folder ")
                        .unwrap_or_default()
                        .into(),
                },
            ),
            "folder has no folder projection",
        )?,
        command if command.starts_with("restore ") => receipt_text(
            submit(
                session,
                CommandAction::RestoreFolder {
                    folder_id: command_argument(command, "restore ")
                        .unwrap_or_default()
                        .into(),
                },
            ),
            "restore has no folder projection",
        )?,
        command if command.starts_with("interrupt ") => {
            let mut arguments = command["interrupt ".len()..].split_whitespace();
            match (arguments.next(), arguments.next(), arguments.next()) {
                (Some(interruption_id), Some(choice), None) => receipt_text(
                    submit(
                        session,
                        CommandAction::ResolveInterruption {
                            interruption_id: interruption_id.into(),
                            choice: choice.into(),
                        },
                    ),
                    "interrupt has no calendar projection",
                )?,
                _ => "invalid interrupt command: expected interrupt <id> <stay|park|close>".into(),
            }
        }
        command if command.starts_with("inspect ") => {
            let requested = command_argument(command, "inspect ").unwrap_or_default();
            let selected = match requested.parse::<usize>() {
                Ok(index) => {
                    let Projection::Book(book) = session.view(View::Book).map_err(failure)? else {
                        return Err(failure("book view is not a Morning Book"));
                    };
                    index
                        .checked_sub(1)
                        .and_then(|index| book.records.get(index))
                        .map(|record| record.record_id.clone())
                        .ok_or_else(|| format!("Morning Book item {requested} does not exist"))
                }
                Err(error) => Err(error.to_string()),
            };
            match selected {
                Ok(record_id) => {
                    let receipt = submit(session, CommandAction::Inspect { record_id });
                    if receipt.accepted {
                        receipt_text(receipt, "inspect has no record projection")?
                    } else {
                        format!(
                            "invalid inspect command: {}",
                            receipt.reason.unwrap_or_else(|| "command rejected".into())
                        )
                    }
                }
                Err(error) => format!("invalid inspect command: {error}"),
            }
        }
        _ => command_banner(session).into(),
    };
    Ok(Input::Output(output))
}

/// Executes the same parser and Session calls as the terminal, without input prompts.
pub(crate) fn scripted(
    scenario: &FrozenScenario,
    package: &str,
    commands: &[&str],
) -> Result<String, ContentError> {
    let mut session = Session::new(scenario, package).map_err(failure)?;
    let mut output = format!(
        "{}\n\n{}\n",
        session.view(View::Book).map_err(failure)?.text(),
        command_banner(&session)
    );
    for command in commands {
        match perform(&mut session, command.trim())? {
            Input::Quit => break,
            Input::Output(text) => {
                output.push_str(&text);
                output.push('\n');
            }
        }
    }
    Ok(output)
}

pub(crate) fn run(args: PlayArgs) -> Result<(), ContentError> {
    let scenario = validate_scenario_with_catalog(&args.source.scenario, &args.source.catalog)?;
    let mut session = Session::new(&scenario, &args.package).map_err(failure)?;
    println!("{}", session.view(View::Book).map_err(failure)?.text());
    if !io::stdin().is_terminal() {
        return Ok(());
    }
    println!("\n{}", command_banner(&session));
    let mut line = String::new();
    loop {
        print!("reservist> ");
        io::stdout().flush()?;
        line.clear();
        if io::stdin().read_line(&mut line)? == 0 {
            break;
        }
        match perform(&mut session, line.trim())? {
            Input::Quit => break,
            Input::Output(text) => println!("{text}"),
        }
    }
    Ok(())
}
