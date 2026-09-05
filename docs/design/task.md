# Reservist - Working Design Spine

## Core Premise

Reservist is a satirical institutional crisis simulator centered on the Federal Reserve Chair.

The player does not control "the economy." They occupy one powerful but bounded node inside a large adaptive system composed of markets, institutions, businesses, political actors, population cohorts, media systems, and external shocks.

The central question is not:

> What policy fixes the economy?

It is:

> Given partial information, institutional constraints, competing mandates, delayed effects, and other actors pursuing their own goals, what do you do - and what system do you leave behind?

The player should be able to understand why individual actors acted as they did without being able to reduce the entire system to a transparent causal diagram.

Locally legible, globally difficult.

### Campaign direction

The first intended world profile opens in early 2006 with Ben Bernankey holding
the Board Chair and FOMC Chair offices. Alan Greenspaniel, Janet Jackrabbit,
Jerome Owl, Kevin Boarsh, and Paul Vulture are stable later-profile candidates,
not participants in the early-2006 campaign. This direction fixes the player
anchor and office continuity only. It does not script a housing crisis, select
a scenario representation manifest, or supply opening state and calibration.


## 1. World Model

The simulation contains a real underlying state, but the player cannot directly inspect it.

Relevant latent variables may include:

- inflationary pressure
- labor-market slack
- dealer balance-sheet elasticity
- bank funding stress
- foreign appetite for U.S. duration
- fiscal-dominance expectations
- financial-system leverage
- household financial stress
- political tolerance for unemployment/inflation
- institutional confidence
- market liquidity

The engine knows some underlying state. Actors know beliefs about that state. The player sees evidence.

This distinction is foundational.

The game should never expose:

> Fiscal credibility: 63/100

Instead, the player sees:

- an ugly Treasury auction
- rising term premium estimates
- foreign reserve flows
- dealer commentary
- widening basis spreads
- political rhetoric
- changing survey responses

The player infers the state of the world.

## 2. Observations and Beliefs

The world produces observable evidence:

- CPI / PCE
- payrolls
- unemployment
- wage growth
- GDP
- SOFR
- repo volume
- SRF usage
- bank balance sheets
- Treasury auction results
- dealer inventories
- Treasury yields
- yield-curve shape
- basis spreads
- mortgage rates
- lending standards
- FX moves
- surveys
- media reporting
- private market chatter

Each agent interprets evidence according to its own priors, models, incentives, and information access.

A belief should approximately carry:

- value
- prior
- confidence
- provenance
- last update
- possibly uncertainty/distribution rather than only a point value

Example:

```text
Fed staff
Fiscal-dominance risk: moderate
Confidence: high
Evidence: staff models, surveys, auctions

Macro hedge fund
Fiscal-dominance risk: high
Confidence: medium
Evidence: auctions, political rhetoric, positioning

Foreign reserve manager
Fiscal-dominance risk: rising
Confidence: low
Evidence: long-term fiscal projections, FX behavior
```

There is no requirement that those estimates converge quickly.

## 3. Actors

The model should distinguish three broad scales.

### Named People

Individuals whose discretion matters.

Examples:

- Fed Chair
- President
- Treasury Secretary
- major central-bank heads
- major sovereign principals
- important regulators
- selected bank/fund executives
- media personalities

A person has:

- material interests
- ideology
- personal priors
- career incentives
- social identity
- political preferences
- institutional roles

A Republican hedge-fund manager is simultaneously:

- a wealthy individual
- a voter
- a homeowner
- perhaps a skier
- a member of a social class
- a professional investor
- the holder of an institutional mandate

Those identities affect different decisions differently.

Institutional rules should strongly constrain professional actions, but personal beliefs can affect interpretation, discretionary choices, and risk tolerance.

### Institutions

These are the main BDI/DSE agents.

Examples:

- Federal Reserve
- Treasury
- White House
- Congress
- banks
- hedge funds
- pensions
- insurers
- money-market funds
- foreign reserve managers
- sovereign wealth funds
- major foreign governments
- central banks
- OPEC actors

Institutions possess:

- balance sheets
- mandates
- legal authority
- action sets
- risk limits
- internal models
- information access
- organizational incentives
- beliefs

### Pops

Vicky-style statistical cohorts representing large groups.

Examples:

- young renters
- affluent homeowners
- retirees
- low-income service workers
- small-business owners
- high-income professionals
- retail traders

Pops generate:

- consumption
- labor supply
- housing demand
- deposit behavior
- sentiment
- voting
- political pressure
- media consumption

They are not individually simulated people.

## 4. Institutional Cognition: BDI + DSE

Clowder's most useful inheritance is the separation between beliefs and action scoring.

BDI supplies the epistemic and intentional context. DSEs determine which available action is attractive under that context.

Example hedge-fund action:

> Expand basis trade

Inputs might include:

- expected basis convergence
- repo funding cost
- margin requirement
- volatility belief
- liquidity buffer
- available risk budget
- gross leverage
- drawdown
- redemption risk
- expected Fed policy
- expected Treasury issuance

No agent needs an action called:

> Respond to Warsh Twist

Instead:

Treasury chooses:

- alter issuance composition
- conduct buybacks
- change auction sizes

Fed chooses:

- issue hawkish guidance
- change policy stance
- alter liquidity facilities

Funds choose:

- increase duration
- decrease duration
- expand basis
- reduce basis
- buy volatility
- raise cash

The "twist trade" is the emergent result of multiple primitive actions.

Historical episodes should not become named branches in the causal kernel.

## 5. Markets

Agents should propose desired positions or actions. Markets determine prices.

This is essential.

The engine should not contain:

> Treasury buys $100B long bonds -> 10Y falls 7 bp.

Instead, the Treasury action changes supply. Other agents change desired holdings. Markets clear.

Long-duration demand may come from:

- pensions
- banks
- insurers
- reserve managers
- asset managers
- leveraged funds
- dealers

Each has different constraints and demand sensitivity.

Then:

```text
sum(DesiredHoldings_i(P)) approximately equals Supply
```

determines price.

This permits policy to "work" mechanically while failing in equilibrium.

Treasury may reduce long-duration supply while simultaneously:

- foreign demand falls
- inflation risk rises
- fiscal risk rises
- private capital demand rises

Result:

> 10-year yield: 4.80%

No special failure script required.

## 6. Three Levels of Causality

### Level 1 - Accounting and Hard Constraints

These should be close to physics.

- assets = liabilities + equity
- cash must come from somewhere
- securities mature
- interest must be paid
- collateral has ownership and eligibility
- margin calls consume liquidity
- borrowing creates obligations
- Treasury issuance creates securities

These rules should not depend on agent beliefs.

### Level 2 - Market Mechanisms

Empirically calibrated mechanisms:

- demand schedules
- market clearing
- repo financing
- haircuts
- dealer intermediation
- liquidity and market impact
- yield-curve behavior
- FX transmission
- credit transmission

These contain uncertainty and empirical approximation.

### Level 3 - Behavioral Response

Deliberately imperfect:

- beliefs
- priors
- risk tolerance
- political preferences
- organizational incentives
- mandates
- precautionary behavior
- career incentives

This is where BDI/DSE lives.

Do not hide Level 1 inside behavioral AI.

## 7. Time

The best structure appears to be:

> continuous/event-driven simulation underneath + FOMC-centered player turns

### Strategic Cadence

The major player turn is the FOMC meeting cycle.

Roughly:

1. previous decision
2. immediate market response
3. intermeeting data
4. institutional adaptation
5. delayed effects
6. political/media response
7. blackout period
8. formal staff briefing
9. next FOMC decision

### Intermeeting Period

The world continues without waiting for the player.

Most agents do not wake every simulation tick.

Agents reconsider actions when:

- relevant evidence arrives
- a price moves materially
- a constraint approaches a boundary
- a scheduled review occurs
- an intention expires
- another actor contacts them

Markets can update much faster than cognition.

Possible cadence:

- intraday: fast-market repricing
- daily: settlement, funding, balances
- weekly: institutional reviews
- scheduled: auctions, reports, speeches
- six-week: FOMC

### Crisis Interrupts

Normal cadence should occasionally break.

Examples:

```text
SUNDAY 18:42
bank failure
repo dysfunction
war
pandemic
currency crisis
Treasury-market disorder
```

The interface itself should communicate that normal institutional time has broken.

## 8. Player Capacity: "Mana"

The player has a finite amount of usable institutional capacity.

This is not stamina.

Actions reserve capacity while they remain active.

Example:

```text
Emergency lending facility
Capacity reserved: 3
Duration: 4 weeks
```

Those points are unavailable while the commitment is active.

When the action expires:

- capacity is released
- successful outcomes may increase future capacity
- failures may reduce future capacity

The player is therefore staking institutional capital on commitments.

### Leash

Maximum usable capacity is determined partly by others' confidence in the Chair.

```text
Capacity_max = Base + f(Credibility, MandatePerformance, Legitimacy, PoliticalSupport)
```

Performance produces leash. Leash enables greater concurrent intervention. Poor performance reduces leash.

Losing leash should generally be easier than gaining it.

A confident public prediction that proves wrong should hurt more than cautious language.

### Commitment and Communication

Public statements can themselves reserve reputation.

Compare:

> "Inflation has moderated, though risks remain."

versus:

> "The Committee is confident inflation is returning sustainably to 2 percent."

The second has greater signaling power. It also collateralizes more future credibility.

Reality can call that loan.

### Loss of Office

Very low leash does not instantly end the game.

Instead:

- markets discount statements
- colleagues dissent
- political pressure rises
- extraordinary actions become difficult
- capacity falls
- institutional independence erodes

Eventually a political/institutional process may produce resignation, replacement, non-renomination, or removal.

Failure screen:

> HIT THE BREAD LINE, JEROME

## 9. Events and "Random Bullshit"

Events should not usually specify their final economic consequences.

They inject shocks into generic transmission channels.

Rule:

> The event specifies the shock. The simulation specifies the consequences.

### Exogenous Shocks

Examples:

- war
- pandemic
- drought
- wildfire
- earthquake
- crop failure
- canal blockage
- cyberattack
- political collapse
- technological breakthrough

Ever Given does not require simulated container ships.

It may inject:

- shipping capacity down
- congestion up
- freight cost up
- delays up

Australian wildfire may inject:

- regional agricultural output down
- insurance losses up

Hormuz closure may inject:

- oil supply down
- shipping insurance up
- geopolitical uncertainty up

The engine propagates these through existing systems.

### Endogenous Crises

These arise from system conditions:

- bank failure
- hedge-fund liquidation
- Treasury auction dysfunction
- deposit run
- market liquidity breakdown
- currency stress

These should generally emerge from continuous processes and probabilistic hazards, not event cards.

A bank should deteriorate through:

- deposit outflows
- liquidity
- capital
- unrealized losses
- collateral
- borrowing capacity
- confidence

Failure is discrete because the legal/institutional boundary eventually becomes discrete.

The path is not.

### Random Bullshit

Some small events should primarily alter beliefs or salience:

- CEO says something stupid
- Fed official gives an unexpected interview
- Treasury leaks a plan
- famous investor announces a giant short
- data release is delayed
- President posts at 2:14 AM
- ship gets stuck somewhere

These provide texture without requiring every event to be an elegant consequence of macro structure.

## 10. World Scope and Flattening

The game should not simulate "the world."

It simulates the interfaces through which the world enters the Fed's problem space.

A useful resolution hierarchy:

### Fully Simulated

Balance sheets + beliefs + actions.

Examples:

- Fed
- Treasury
- banks
- hedge funds
- major sovereign actors

### Mechanistically Aggregated

State + rules, but no cognition.

Examples:

- housing
- labor
- agriculture
- shipping
- energy markets

### Exogenous Generators

Shock producers.

Examples:

- weather
- earthquakes
- outbreak initiation
- geopolitical incidents

This allows asymmetric geographic resolution.

"Europe" may often be adequately modeled as:

- growth
- ECB policy
- EUR/USD
- sovereign stress
- aggregate demand
- energy demand

while the Middle East may remain much more granular because Saudi Arabia, UAE, Iran, Qatar, Israel, and others possess materially distinct action sets and transmission channels.

Granularity follows causal relevance, not cartographic symmetry.

## 11. Historical Research

The purpose of historical research is system identification, not reenactment.

Study:

- SVB
- 2022 LDI
- March 2020 Treasury dysfunction
- 2019 repo spike
- COVID
- inflation episodes
- oil shocks
- sovereign crises
- historical Fed communications failures

For each episode ask:

- what state variables mattered?
- what balance-sheet constraints mattered?
- what actors had which feasible actions?
- what information arrived when?
- what loops activated?
- what delays mattered?
- what behavior recurred across institutions?
- what was historically contingent?

The validation criterion is not:

> Does the simulation reproduce history exactly?

It is:

> Under historically plausible initial conditions, can the simulation produce the same class of mechanism and response?

Historical episodes become integration tests.

## 12. Vibecession and Mass Perception

The simulation must distinguish:

```text
MaterialEconomy -> ExperiencedEconomy -> InformationEnvironment -> ReportedBelief
```

A pop may experience employment and rising real wages while simultaneously experiencing elevated rent, high mortgage rates, expensive groceries, childcare costs, and poor housing mobility.

Its belief that "the economy is bad" can therefore be internally coherent even during good aggregate GDP growth.

Perception is shaped by:

- actual material experience
- salient price levels
- expectations
- peer information
- ideology
- media
- institutional trust

This allows sentiment to diverge from headline aggregates without a `VibecessionEvent`.

## 13. Media and Information Ecosystem

The game's information environment is itself simulated.

### Loonberg

High-quality financial wire/terminal.

- fast
- institutionally connected
- terse
- usually reliable

> HONK - UST 10Y AUCTION TAILS 8.7BP

### The Herd

Private professional information network. Bloomberg-chat / Signal / WhatsApp equivalent.

- fastest market rumor
- narrow audience
- high informational value
- uncertain verification

> "Hearing one large fox shop cutting 10s."

### HonkBox

Twitter/X.

- maximal speed
- primary-source posts
- rumors
- bots
- screenshots
- shitposts
- political signaling

### GNBC

Great Nation Broadcasting Channel. Cable-news ecosystem.

- mass salience
- partisan framing
- political feedback
- slower than HonkBox

### GooseTogetherStrong

Retail-investor slop.

- options mania
- meme stocks
- conspiracy narratives
- accidental insight
- extreme amplification

> THEY CANNOT MARGIN CALL US ALL

### Wool Street Journal

Establishment financial press.

- elite consensus
- Fed signaling
- slower, authoritative interpretation

### AFTV

Alpaca Financial Television. CNBC analogue.

- CEOs
- portfolio managers
- instant narrative construction
- theatrical certainty

### Media Mechanics

A claim should carry:

- proposition
- source
- hidden truth relation
- evidence strength
- novelty
- ambiguity
- salience
- distribution history

An outlet carries:

- audience
- speed
- access
- accuracy
- sensationalism
- ideological tendencies
- topic preferences
- amplification behavior

Media should transform information, not simply display it.

The same underlying event might move through:

```text
The Herd -> Loonberg -> HonkBox -> AFTV/GNBC -> mass public belief
```

Sometimes that order reverses.

## 14. Art and Tone

The world is represented by animals.

The target aesthetic is approximately:

> Papers, Please-like restrained pixel art + 1980s institutional finance + editorial-cartoon animal satire.

Core visual scene:

- dark desk
- green accounting lamps
- beige memos
- terminals
- dot-matrix charts
- institutional clutter
- sparse animation

Characters are animals in professional dress.

Examples:

- goose markets-desk analyst
- chimp finance-TV host loosely evoking Jim Cramer
- fox hedge-fund manager
- owl central banker
- camel oil minister
- wool/alpaca establishment financier

Species should provide visual suggestion rather than deterministic morality.

The satire should live in:

- names
- species
- props
- media framing
- pompous institutional language
- contradictory rhetoric

Serious outcomes should remain serious.

The world is funny. The consequences are not.

## 15. Design Principle for Legibility

The player should be able to inspect:

> Why did this actor do this?

Example:

```text
MACRO FUND 7
Selected: Reduce long duration

Drivers:
- fiscal-risk expectation
- inflation-path expectation
- portfolio drawdown
- Treasury supply outlook

Against:
- attractive current yield
- mean-reversion prior

Constraints:
- leverage near limit
- weak liquidity buffer

Beliefs:
- 12m policy rate: 4.2%, medium confidence
- fiscal risk: high, low confidence
```

That is legible.

The player should not necessarily be able to answer:

> Why exactly did the 10-year move 23 bp today?

That is the aggregate result of many locally explicable decisions.

## 16. What Not to Do

Avoid:

- `if TwistTrade`
- `if Vibecession`
- `if SVB`
- historical event-response scripts
- LLMs deciding the causal outcome of markets
- omniscient agents
- one universal representative investor
- one "economy health" score
- one global mana currency explaining every constraint
- simulating unnecessary physical detail
- making every country equally detailed

LLMs may safely assist with:

- rendering structured events into news copy
- phone dialogue
- staff memos
- flavor variation
- post-hoc explanation

They should not determine the simulation's physical/economic outcome.

## 17. Current Technical Hypothesis

The emerging architecture is:

```text
LATENT WORLD STATE
        |
        v
OBSERVABLE EVIDENCE / PRIVATE SIGNALS
        |
        v
PER-AGENT BELIEF UPDATES
        |
        v
BDI CONTEXT
        |
        v
DSE ACTION SCORING
        |
        v
PERSISTENT INTENTIONS / ORDERS / POLICY COMMITMENTS
        |
        v
MARKET CLEARING + ACCOUNTING + MECHANICAL SYSTEMS
        |
        v
DELAYED EFFECTS
        |
        v
NEW WORLD STATE
```

External shocks enter the world state/mechanism layer. Media transforms evidence and beliefs. Pops transform lived economic conditions into sentiment and politics. The player occupies one institutional node inside the graph.

The player never gets `World`. The player gets an inbox.

## 18. Design Closure

The questions formerly collected here no longer form an active architecture
backlog. The later design discussions resolve their contract-level content:

- Market topology uses dependency-closed slices and typed adapters. Exact solvers,
  equations, and calibration remain implementation work.
- Agent resolution uses closed fidelity tiers and promotion by distinct material
  action ownership, not a target global agent count.
- Beliefs use typed bounded estimates, source ledgers, uncertainty, provenance, and
  recipient-scoped evidence delivery. Exact update mathematics remain testable
  mechanism work.
- Population and geography use scenario-bounded representation, exact ownership and
  residual reconciliation, protected tails, and pre-run promotion.
- Leash is a derived player-facing projection over typed institutional commitments
  and constraints, not a canonical resource account.
- Communications use structured claims whose rendered prose remains presentational.
- Historical validation uses hard invariants, mechanism predicates, deterministic
  replay, and multi-seed outcome bands rather than exact reenactment.
- Epistemic fairness uses embodied institutional routing, bounded requests, visible
  displaced work, source-specific uncertainty, and in-world staff postmortems.
- Failure changes authority and institutional history before resignation, removal,
  incapacity, or term expiry ends the playable role.

The remaining gates are narrower: complete the Treasury basis-trade composition
trace, define the machine-readable scenario manifest and initialization contract,
close only the selected first-slice catalog entries, and implement and calibrate the
two first proofs described in the master architecture. Broader catalog population is
not permission to widen the initial runtime.

## North Star

Reservist should not attempt to answer what the correct monetary policy is.

It should construct a materially coherent institutional system in which:

- actors have incomplete beliefs
- everyone has constraints
- markets aggregate incompatible intentions
- shocks enter through real transmission channels
- policy creates second-order effects
- media changes beliefs
- material experience creates politics
- institutions remember prior performance
- the player must act without ever possessing the complete causal model

Or, in shorter form:

> Nobody understands the whole machine. Every local action has a reason. The machine remembers what happened.
