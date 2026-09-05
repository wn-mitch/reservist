# 08-inventory-closure-and-mvp-slices

- conceptual scope: Closure inventory, Federal Reserve/Treasury inventory, and MVP phase inventory slices
- contained artifacts: 64
- source omnibus: `humanlayer-omni.md`
- preservation: artifact headings, YAML metadata, and payload bytes below are preserved from the source omnibus; outer Markdown fence delimiters are reselected collision-safely for compact model ingestion.

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/closure/action_domains.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/closure/action_domains.csv
size_bytes: 36
mode_octal: "0644"
modified_at_utc: 2026-09-04T04:04:58.401928Z
sha256: 58659ae1aa6b2c42db70b4b345f09bbf83bd18316a913b47d9c87ae0b7353a10
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
catalog_id,action_domain,provenance
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/closure/composition_probe_members.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/closure/composition_probe_members.csv
size_bytes: 8746
mode_octal: "0644"
modified_at_utc: 2026-09-04T18:32:41.158904Z
sha256: 40e4358a944a3b83f3191c125afabd7f88dda2bc0761e95a700a3a99b6d88f02
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
composition_probe_id,catalog_id,candidate_provider_entry_id,selected_fidelity,requirement,provenance
probe.composition.burrow_bank,inst.us.bank.burrow,cohort.us.bank.regional_cre_concentrated,PARTICIPANT_DISTRIBUTION,Structural comparison member; membership does not grant scenario availability or catalog eligibility.,07-design-discussion-burrow-composition-probe.md
probe.composition.burrow_bank,cohort.us.bank.regional_cre_concentrated,mechanism.us.bank.regional_aggregate,ORGANIZATION_COHORT_RESPONSE,Structural comparison member; membership does not grant scenario availability or catalog eligibility.,07-design-discussion-burrow-composition-probe.md
probe.composition.burrow_bank,mechanism.us.bank.regional_aggregate,NONE,MECHANICAL_OR_ADAPTER,Structural comparison member; membership does not grant scenario availability or catalog eligibility.,07-design-discussion-burrow-composition-probe.md
probe.composition.burrow_bank,facility.us.federal_reserve.discount_window,NONE,MECHANICAL_OR_ADAPTER,Structural comparison member; membership does not grant scenario availability or catalog eligibility.,07-design-discussion-burrow-composition-probe.md
probe.composition.burrow_bank,mechanism.us.payment.fedwire_funds,NONE,MECHANICAL_OR_ADAPTER,Structural comparison member; membership does not grant scenario availability or catalog eligibility.,07-design-discussion-burrow-composition-probe.md
probe.composition.burrow_bank,mechanism.us.settlement.fedwire_securities,NONE,MECHANICAL_OR_ADAPTER,Structural comparison member; membership does not grant scenario availability or catalog eligibility.,07-design-discussion-burrow-composition-probe.md
probe.composition.burrow_bank,type.legal_instrument.deposit_insurance_condition,NONE,MECHANICAL_OR_ADAPTER,Structural comparison member; membership does not grant scenario availability or catalog eligibility.,07-design-discussion-burrow-composition-probe.md
probe.composition.burrow_bank,type.record.call_report,NONE,MECHANICAL_OR_ADAPTER,Structural comparison member; membership does not grant scenario availability or catalog eligibility.,07-design-discussion-burrow-composition-probe.md
probe.composition.treasury_basis_trade,inst.us.treasury,NONE,PARTICIPANT_DISTRIBUTION,Structural comparison member; membership does not grant scenario availability or catalog eligibility.,Representation Catalog Treasury-duration dependency cut.
probe.composition.treasury_basis_trade,market.us.treasury.secondary,NONE,MECHANICAL_OR_ADAPTER,Structural comparison member; membership does not grant scenario availability or catalog eligibility.,Representation Catalog Treasury-duration dependency cut.
probe.composition.treasury_basis_trade,market.us.treasury_futures,NONE,MECHANICAL_OR_ADAPTER,Structural comparison member; membership does not grant scenario availability or catalog eligibility.,Representation Catalog Treasury-duration dependency cut.
probe.composition.treasury_basis_trade,mechanism.us.repo.tri_party,NONE,MECHANICAL_OR_ADAPTER,Structural comparison member; membership does not grant scenario availability or catalog eligibility.,Representation Catalog Treasury-duration dependency cut.
probe.composition.treasury_basis_trade,inst.us.clearing.ficc,NONE,PARTICIPANT_DISTRIBUTION,Structural comparison member; membership does not grant scenario availability or catalog eligibility.,Representation Catalog Treasury-duration dependency cut.
probe.composition.treasury_basis_trade,mechanism.us.clearing.ficc_repo,NONE,MECHANICAL_OR_ADAPTER,Structural comparison member; membership does not grant scenario availability or catalog eligibility.,Representation Catalog Treasury-duration dependency cut.
probe.composition.treasury_basis_trade,cohort.us.dealer.primary,NONE,PARTICIPANT_DISTRIBUTION,Structural comparison member; membership does not grant scenario availability or catalog eligibility.,Representation Catalog Treasury-duration dependency cut.
probe.composition.treasury_basis_trade,inst.us.fund.macro_fund_7,NONE,PARTICIPANT_DISTRIBUTION,Structural comparison member; membership does not grant scenario availability or catalog eligibility.,Representation Catalog Treasury-duration dependency cut.
probe.composition.treasury_basis_trade,inst.us.federal_reserve.board,NONE,PARTICIPANT_DISTRIBUTION,Structural comparison member; membership does not grant scenario availability or catalog eligibility.,Representation Catalog Treasury-duration dependency cut.
probe.composition.treasury_basis_trade,institution.us.new_york_fed,NONE,PARTICIPANT_DISTRIBUTION,Structural comparison member; membership does not grant scenario availability or catalog eligibility.,Representation Catalog Treasury-duration dependency cut.
probe.composition.treasury_basis_trade,reference.us.treasury.par_yield_curve,NONE,MECHANICAL_OR_ADAPTER,Structural comparison member; membership does not grant scenario availability or catalog eligibility.,Representation Catalog Treasury-duration dependency cut.
probe.composition.treasury_basis_trade,reference.us.nyfed.sofr,NONE,MECHANICAL_OR_ADAPTER,Structural comparison member; membership does not grant scenario availability or catalog eligibility.,Representation Catalog Treasury-duration dependency cut.
probe.composition.amazon_region_replacement,region.sa.amazon_basin,NONE,MECHANICAL_OR_ADAPTER,Structural comparison member; membership does not grant scenario availability or catalog eligibility.,Representation Catalog worked Amazon chain.
probe.composition.amazon_region_replacement,generator.climate.amazon_basin,adapter.external.external_world,MECHANICAL_OR_ADAPTER,Structural comparison member; membership does not grant scenario availability or catalog eligibility.,Representation Catalog worked Amazon chain.
probe.composition.amazon_region_replacement,process.climate_agriculture.amazon_basin,adapter.external.external_world,MECHANICAL_OR_ADAPTER,Structural comparison member; membership does not grant scenario availability or catalog eligibility.,Representation Catalog worked Amazon chain.
probe.composition.amazon_region_replacement,industry.amazon_basin.forestry,adapter.external.external_world,ORGANIZATION_COHORT_RESPONSE,Structural comparison member; membership does not grant scenario availability or catalog eligibility.,Representation Catalog worked Amazon chain.
probe.composition.amazon_region_replacement,industry.amazon_basin.coffee,adapter.external.external_world,ORGANIZATION_COHORT_RESPONSE,Structural comparison member; membership does not grant scenario availability or catalog eligibility.,Representation Catalog worked Amazon chain.
probe.composition.amazon_region_replacement,industry.amazon_basin.pork,adapter.external.external_world,ORGANIZATION_COHORT_RESPONSE,Structural comparison member; membership does not grant scenario availability or catalog eligibility.,Representation Catalog worked Amazon chain.
probe.composition.amazon_region_replacement,mechanism.transform.global.timber_milling,adapter.external.external_world,MECHANICAL_OR_ADAPTER,Structural comparison member; membership does not grant scenario availability or catalog eligibility.,Representation Catalog worked Amazon chain.
probe.composition.amazon_region_replacement,mechanism.transform.global.coffee_roasting,adapter.external.external_world,MECHANICAL_OR_ADAPTER,Structural comparison member; membership does not grant scenario availability or catalog eligibility.,Representation Catalog worked Amazon chain.
probe.composition.amazon_region_replacement,mechanism.transform.global.hogs_slaughter,adapter.external.external_world,MECHANICAL_OR_ADAPTER,Structural comparison member; membership does not grant scenario availability or catalog eligibility.,Representation Catalog worked Amazon chain.
probe.composition.amazon_region_replacement,market.product.global.timber,adapter.external.external_world,MECHANICAL_OR_ADAPTER,Structural comparison member; membership does not grant scenario availability or catalog eligibility.,Representation Catalog worked Amazon chain.
probe.composition.amazon_region_replacement,market.product.global.coffee,adapter.external.external_world,MECHANICAL_OR_ADAPTER,Structural comparison member; membership does not grant scenario availability or catalog eligibility.,Representation Catalog worked Amazon chain.
probe.composition.amazon_region_replacement,market.product.global.pork,adapter.external.external_world,MECHANICAL_OR_ADAPTER,Structural comparison member; membership does not grant scenario availability or catalog eligibility.,Representation Catalog worked Amazon chain.
probe.composition.amazon_region_replacement,adapter.external.external_world,NONE,MECHANICAL_OR_ADAPTER,Structural comparison member; membership does not grant scenario availability or catalog eligibility.,Representation Catalog worked Amazon chain.
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/closure/composition_probe_requirements.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/closure/composition_probe_requirements.csv
size_bytes: 4413
mode_octal: "0644"
modified_at_utc: 2026-09-04T18:32:41.157468Z
sha256: 43d9db807199e9e32c6f2dd06e87b190aa056e2093c71f39177f7cdb76cfd1e9
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
composition_probe_id,architecture_probe_id,catalog_id,gate,required_status,provenance
probe.composition.burrow_bank,probe.type_instance_compatibility,inst.us.bank.burrow,catalog_contract,covered,Representation Catalog composition-probe contract.
probe.composition.burrow_bank,probe.owner_witness_chain,inst.us.bank.burrow,catalog_contract,covered,Representation Catalog composition-probe contract.
probe.composition.burrow_bank,probe.fallback_parity,cohort.us.bank.regional_cre_concentrated,catalog_contract,covered,Representation Catalog composition-probe contract.
probe.composition.burrow_bank,probe.instrument_account_contract,mechanism.us.bank.regional_aggregate,catalog_contract,covered,Representation Catalog composition-probe contract.
probe.composition.burrow_bank,probe.legal_information_path,type.legal_instrument.deposit_insurance_condition,catalog_contract,covered,Representation Catalog composition-probe contract.
probe.composition.burrow_bank,init.period_content,inst.us.bank.burrow,runtime_initialization,blocked,Representation Catalog documented runtime blocker.
probe.composition.burrow_bank,init.legal_content,inst.us.bank.burrow,runtime_initialization,blocked,Representation Catalog documented runtime blocker.
probe.composition.burrow_bank,init.opening_accounts,inst.us.bank.burrow,runtime_initialization,blocked,Representation Catalog documented runtime blocker.
probe.composition.burrow_bank,init.residual_values,inst.us.bank.burrow,runtime_initialization,blocked,Representation Catalog documented runtime blocker.
probe.composition.burrow_bank,init.calibration,inst.us.bank.burrow,runtime_initialization,blocked,Representation Catalog documented runtime blocker.
probe.composition.treasury_basis_trade,probe.treasury_basis_graph,market.us.treasury.secondary,catalog_contract,covered,Representation Catalog composition-probe contract.
probe.composition.treasury_basis_trade,probe.settlement_and_clearing,inst.us.clearing.ficc,catalog_contract,covered,Representation Catalog composition-probe contract.
probe.composition.treasury_basis_trade,probe.instrument_account_contract,mechanism.us.repo.tri_party,catalog_contract,covered,Representation Catalog composition-probe contract.
probe.composition.treasury_basis_trade,probe.binding_publication,reference.us.nyfed.sofr,catalog_contract,covered,Representation Catalog composition-probe contract.
probe.composition.treasury_basis_trade,init.opening_positions,inst.us.treasury,runtime_initialization,blocked,Representation Catalog documented runtime blocker.
probe.composition.treasury_basis_trade,init.counterparty_content,inst.us.treasury,runtime_initialization,blocked,Representation Catalog documented runtime blocker.
probe.composition.treasury_basis_trade,init.market_parameters,inst.us.treasury,runtime_initialization,blocked,Representation Catalog documented runtime blocker.
probe.composition.treasury_basis_trade,init.calibration,inst.us.treasury,runtime_initialization,blocked,Representation Catalog documented runtime blocker.
probe.composition.amazon_region_replacement,probe.region_process_separation,region.sa.amazon_basin,catalog_contract,covered,Representation Catalog composition-probe contract.
probe.composition.amazon_region_replacement,probe.product_conservation,process.climate_agriculture.amazon_basin,catalog_contract,covered,Representation Catalog composition-probe contract.
probe.composition.amazon_region_replacement,probe.named_residual_replacement,adapter.external.external_world,catalog_contract,covered,Representation Catalog composition-probe contract.
probe.composition.amazon_region_replacement,probe.no_magic_price,market.product.global.timber,catalog_contract,covered,Representation Catalog composition-probe contract.
probe.composition.amazon_region_replacement,init.opening_physical_state,region.sa.amazon_basin,runtime_initialization,blocked,Representation Catalog documented runtime blocker.
probe.composition.amazon_region_replacement,init.regional_shares,region.sa.amazon_basin,runtime_initialization,blocked,Representation Catalog documented runtime blocker.
probe.composition.amazon_region_replacement,init.process_parameters,region.sa.amazon_basin,runtime_initialization,blocked,Representation Catalog documented runtime blocker.
probe.composition.amazon_region_replacement,init.calibration,region.sa.amazon_basin,runtime_initialization,blocked,Representation Catalog documented runtime blocker.
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/closure/composition_probes.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/closure/composition_probes.csv
size_bytes: 779
mode_octal: "0644"
modified_at_utc: 2026-09-04T04:03:52.976173Z
sha256: bc1821070d89c876335dacbced864ab6e0dda1a3d80b378d579436154bd95396
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
composition_probe_id,display_name,purpose,provenance
probe.composition.burrow_bank,Burrow bank replacement,"Compare named bank, regional cohort, and aggregate mechanism while exercising institutional, legal, settlement, account, and information paths.",07-design-discussion-burrow-composition-probe.md
probe.composition.treasury_basis_trade,Treasury basis trade,"Exercise cash Treasury, repo, futures, reserve, equity, clearing, account, Fed-system, and publication contracts.",Representation Catalog Treasury-duration dependency cut.
probe.composition.amazon_region_replacement,Amazon region replacement,"Exercise scope, generator, process, product stages, named-plus-residual replacement, conservation, and no-magic-price behavior.",Representation Catalog worked Amazon chain.
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/closure/entities.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/closure/entities.csv
size_bytes: 4537
mode_octal: "0644"
modified_at_utc: 2026-09-04T04:03:06.735005Z
sha256: 035bce10fc5480d78e26605c0e218ba8852f2d2b640d8e00f75f5aa7341d34de
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
catalog_id,display_name,entry_class,instance_of,identity_clade,domain,selectable_in_manifest,definition_version,cognition_class,default_fidelity,research_status,completeness_state,fit,fallback_entry_id,residual_counterpart_id,uncertainty_notes,provenance
adapter.incident.external.chokepoint_blockage,External chokepoint blockage adapter,instance,type.boundary_adapter.default,BoundaryAdapter,physical_products,false,1,none,MECHANICAL_OR_ADAPTER,typed,typed,fits,UNKNOWN,NONE,Interface and output distribution UNKNOWN.,05-design-discussion-representation-bible.md:731-735
adapter.incident.external.cyber,External cyber incident adapter,instance,type.boundary_adapter.default,BoundaryAdapter,sovereign_regional,false,1,none,MECHANICAL_OR_ADAPTER,typed,typed,fits,UNKNOWN,NONE,Interface and output distribution UNKNOWN.,05-design-discussion-representation-bible.md:731-735
adapter.incident.external.drought,External drought adapter,instance,type.boundary_adapter.default,BoundaryAdapter,physical_products,false,1,none,MECHANICAL_OR_ADAPTER,typed,typed,fits,UNKNOWN,NONE,Interface and output distribution UNKNOWN.,05-design-discussion-representation-bible.md:731-735
adapter.incident.external.gaffe_leak,External gaffe or leak adapter,instance,type.boundary_adapter.default,BoundaryAdapter,media_information,false,1,none,MECHANICAL_OR_ADAPTER,typed,typed,fits,UNKNOWN,NONE,Interface and output distribution UNKNOWN.,05-design-discussion-representation-bible.md:731-735
adapter.incident.external.geological,External geological incident adapter,instance,type.boundary_adapter.default,BoundaryAdapter,physical_products,false,1,none,MECHANICAL_OR_ADAPTER,typed,typed,fits,UNKNOWN,NONE,Interface and output distribution UNKNOWN.,05-design-discussion-representation-bible.md:731-735
adapter.incident.external.outbreak,External outbreak adapter,instance,type.boundary_adapter.default,BoundaryAdapter,sovereign_regional,false,1,none,MECHANICAL_OR_ADAPTER,typed,typed,fits,UNKNOWN,NONE,Interface and output distribution UNKNOWN.,05-design-discussion-representation-bible.md:731-735
adapter.incident.external.statistical_release_failure,External statistical release failure adapter,instance,type.boundary_adapter.default,BoundaryAdapter,media_information,false,1,none,MECHANICAL_OR_ADAPTER,typed,typed,fits,UNKNOWN,NONE,Interface and output distribution UNKNOWN.,05-design-discussion-representation-bible.md:731-735
adapter.incident.external.technological_breakthrough,External technological breakthrough adapter,instance,type.boundary_adapter.default,BoundaryAdapter,physical_products,false,1,none,MECHANICAL_OR_ADAPTER,typed,typed,fits,UNKNOWN,NONE,Interface and output distribution UNKNOWN.,05-design-discussion-representation-bible.md:731-735
adapter.incident.external.weather,External weather adapter,instance,type.boundary_adapter.default,BoundaryAdapter,physical_products,false,1,none,MECHANICAL_OR_ADAPTER,typed,typed,fits,UNKNOWN,NONE,Interface and output distribution UNKNOWN.,05-design-discussion-representation-bible.md:731-735
adapter.incident.external.wildfire,External wildfire adapter,instance,type.boundary_adapter.default,BoundaryAdapter,physical_products,false,1,none,MECHANICAL_OR_ADAPTER,typed,typed,fits,UNKNOWN,NONE,Interface and output distribution UNKNOWN.,05-design-discussion-representation-bible.md:731-735
adapter.process.climate_agriculture,Climate and agriculture process adapter,instance,type.boundary_adapter.default,BoundaryAdapter,physical_products,false,1,none,MECHANICAL_OR_ADAPTER,typed,typed,fits,UNKNOWN,NONE,Preserved process outputs and state initialization UNKNOWN.,05-design-discussion-representation-bible.md:731-735
adapter.process.epidemic,Epidemic process adapter,instance,type.boundary_adapter.default,BoundaryAdapter,sovereign_regional,false,1,none,MECHANICAL_OR_ADAPTER,typed,typed,fits,UNKNOWN,NONE,Preserved process outputs and state initialization UNKNOWN.,05-design-discussion-representation-bible.md:731-735
adapter.process.war,War process adapter,instance,type.boundary_adapter.default,BoundaryAdapter,sovereign_regional,false,1,none,MECHANICAL_OR_ADAPTER,typed,typed,fits,UNKNOWN,NONE,Preserved process outputs and state initialization UNKNOWN.,05-design-discussion-representation-bible.md:731-735
region.cn.aggregate,China aggregate region,instance,type.region.default,Region,sovereign_regional,false,1,none,MECHANICAL_OR_ADAPTER,typed,identity_only,fits,UNKNOWN,NONE,Geographic scope only; owns no stocks prices or cognition by geography.,06-design-discussion-representation-catalog.md:368
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/closure/entity_scopes.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/closure/entity_scopes.csv
size_bytes: 9964
mode_octal: "0644"
modified_at_utc: 2026-09-04T04:17:49.344152Z
sha256: 9b3216c80d7499477faf8d9b21211d2c97bbae01145f24f940d0c853313be288
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
catalog_id,scope_entry_id,provenance
adapter.input.amazon_basin.feed_grain,region.sa.aggregate,06-design-discussion-representation-catalog.md:462
adapter.input.amazon_basin.feed_grain,region.sa.amazon_basin,06-design-discussion-representation-catalog.md:462
body.il.cabinet,sovereign.israel,04-design-discussion-minimum-simulation-kernel.md:1346
body.jp.cabinet,sovereign.japan,04-design-discussion-minimum-simulation-kernel.md:1350
coalition.il.governing_parties,sovereign.israel,04-design-discussion-minimum-simulation-kernel.md:1346
coalition.ir.factional_networks,sovereign.iran,04-design-discussion-minimum-simulation-kernel.md:1347
coalition.ru.elite_networks,sovereign.russia,04-design-discussion-minimum-simulation-kernel.md:1352
coalition.uk.parties,sovereign.united_kingdom,04-design-discussion-minimum-simulation-kernel.md:1351
cohort.eu.exposed_banks,federated.euro_area,04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
cohort.jp.banks,sovereign.japan,04-design-discussion-minimum-simulation-kernel.md:1350
cohort.jp.insurers,sovereign.japan,04-design-discussion-minimum-simulation-kernel.md:1350
cohort.jp.pensions,sovereign.japan,04-design-discussion-minimum-simulation-kernel.md:1350
cohort.ru.energy_firms,sovereign.russia,04-design-discussion-minimum-simulation-kernel.md:1352
cohort.sg.dollar_funding_banks,sovereign.singapore,04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
cohort.uk.pensions,sovereign.united_kingdom,04-design-discussion-minimum-simulation-kernel.md:1351
firm.sa.saudi_aramco,sovereign.saudi_arabia,04-design-discussion-minimum-simulation-kernel.md:1345
industry.amazon_basin.coffee,region.sa.amazon_basin,06-design-discussion-representation-catalog.md:461
industry.amazon_basin.forestry,region.sa.amazon_basin,06-design-discussion-representation-catalog.md:461
industry.amazon_basin.pork,region.sa.amazon_basin,06-design-discussion-representation-catalog.md:461
industry.sa.coffee.aggregate,region.sa.aggregate,06-design-discussion-representation-catalog.md:512
industry.sa.forestry.aggregate,region.sa.aggregate,06-design-discussion-representation-catalog.md:512
industry.sa.pork.aggregate,region.sa.aggregate,06-design-discussion-representation-catalog.md:512
inst.us.bank.burrow,region.us.home_district,06-design-discussion-representation-catalog.md:818-904;07-design-discussion-burrow-composition-probe.md
institution.ae.central_bank,sovereign.uae,04-design-discussion-minimum-simulation-kernel.md:1353
institution.ae.energy_companies,sovereign.uae,04-design-discussion-minimum-simulation-kernel.md:1353
institution.ae.sovereign_funds,sovereign.uae,04-design-discussion-minimum-simulation-kernel.md:1353
institution.cn.military,sovereign.china,04-design-discussion-minimum-simulation-kernel.md:1348
institution.cn.party_leadership,sovereign.china,04-design-discussion-minimum-simulation-kernel.md:1348
institution.cn.pboc,sovereign.china,04-design-discussion-minimum-simulation-kernel.md:1348
institution.cn.policy_banks,sovereign.china,04-design-discussion-minimum-simulation-kernel.md:1348
institution.cn.safe,sovereign.china,04-design-discussion-minimum-simulation-kernel.md:1348
institution.cn.state_council,sovereign.china,04-design-discussion-minimum-simulation-kernel.md:1348
institution.de.government,sovereign.germany,04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
institution.eu.commission,federated.euro_area,04-design-discussion-minimum-simulation-kernel.md:1349
institution.eu.ecb,federated.euro_area,04-design-discussion-minimum-simulation-kernel.md:1349
institution.eu.member_governments,federated.euro_area,04-design-discussion-minimum-simulation-kernel.md:1349
institution.eu.national_central_banks,federated.euro_area,04-design-discussion-minimum-simulation-kernel.md:1349
institution.fr.government,sovereign.france,04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
institution.il.bank_of_israel,sovereign.israel,04-design-discussion-minimum-simulation-kernel.md:1346
institution.il.security,sovereign.israel,04-design-discussion-minimum-simulation-kernel.md:1346
institution.ir.central_bank,sovereign.iran,04-design-discussion-minimum-simulation-kernel.md:1347
institution.ir.irgc,sovereign.iran,04-design-discussion-minimum-simulation-kernel.md:1347
institution.ir.oil,sovereign.iran,04-design-discussion-minimum-simulation-kernel.md:1347
institution.it.government,sovereign.italy,04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
institution.jp.boj,sovereign.japan,04-design-discussion-minimum-simulation-kernel.md:1350
institution.jp.mof,sovereign.japan,04-design-discussion-minimum-simulation-kernel.md:1350
institution.qa.central_bank,sovereign.qatar,04-design-discussion-minimum-simulation-kernel.md:1353
institution.qa.energy_companies,sovereign.qatar,04-design-discussion-minimum-simulation-kernel.md:1353
institution.qa.sovereign_funds,sovereign.qatar,04-design-discussion-minimum-simulation-kernel.md:1353
institution.ru.central_bank,sovereign.russia,04-design-discussion-minimum-simulation-kernel.md:1352
institution.ru.presidency,sovereign.russia,04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
institution.ru.security,sovereign.russia,04-design-discussion-minimum-simulation-kernel.md:1352
institution.sa.energy_ministry,sovereign.saudi_arabia,04-design-discussion-minimum-simulation-kernel.md:1345
institution.sa.sama,sovereign.saudi_arabia,04-design-discussion-minimum-simulation-kernel.md:1345
institution.sa.sovereign_funds,sovereign.saudi_arabia,04-design-discussion-minimum-simulation-kernel.md:1345
institution.sg.mas,sovereign.singapore,04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
institution.sg.shipping,sovereign.singapore,04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
institution.sg.sovereign_funds,sovereign.singapore,04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
institution.uk.boe,sovereign.united_kingdom,04-design-discussion-minimum-simulation-kernel.md:1351
institution.uk.dmo,sovereign.united_kingdom,04-design-discussion-minimum-simulation-kernel.md:1351
institution.uk.government,sovereign.united_kingdom,04-design-discussion-minimum-simulation-kernel.md:1351
mechanism.cross_border.chokepoint_shipping,region.hormuz,04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
person.cn.pboc_governor,sovereign.china,06-design-discussion-representation-catalog.md:251
person.cn.safe_head,sovereign.china,06-design-discussion-representation-catalog.md:251
person.cn.xi,sovereign.china,06-design-discussion-representation-catalog.md:251
person.eu.ecb_president,federated.euro_area,06-design-discussion-representation-catalog.md:251
person.il.boi_governor,sovereign.israel,06-design-discussion-representation-catalog.md:251
person.il.prime_minister,sovereign.israel,06-design-discussion-representation-catalog.md:251
person.ir.central_bank_head,sovereign.iran,06-design-discussion-representation-catalog.md:251
person.ir.irgc_head,sovereign.iran,06-design-discussion-representation-catalog.md:251
person.ir.president,sovereign.iran,06-design-discussion-representation-catalog.md:251
person.ir.supreme_leader,sovereign.iran,06-design-discussion-representation-catalog.md:251
person.jp.boj_governor,sovereign.japan,06-design-discussion-representation-catalog.md:251
person.jp.mof_international_affairs_vice_minister,sovereign.japan,06-design-discussion-representation-catalog.md:251
person.sa.aramco_ceo,sovereign.saudi_arabia,06-design-discussion-representation-catalog.md:251
person.sa.crown,sovereign.saudi_arabia,06-design-discussion-representation-catalog.md:251
person.sa.pif_head,sovereign.saudi_arabia,06-design-discussion-representation-catalog.md:251
person.sa.sama_governor,sovereign.saudi_arabia,06-design-discussion-representation-catalog.md:251
person.uk.boe_governor,sovereign.united_kingdom,06-design-discussion-representation-catalog.md:251
person.uk.dmo_chief,sovereign.united_kingdom,06-design-discussion-representation-catalog.md:251
process.climate_agriculture.amazon_basin,region.sa.amazon_basin,06-design-discussion-representation-catalog.md:460
region.cn.provinces,sovereign.china,04-design-discussion-minimum-simulation-kernel.md:1348
region.hormuz,region.middle_east.gulf,06-design-discussion-representation-catalog.md:370
region.sa.amazon_basin,region.sa.aggregate,06-design-discussion-representation-catalog.md:458;06-design-discussion-representation-catalog.md:510
region.us.census_divisions,region.us.aggregate,06-design-discussion-representation-catalog.md:368
region.us.federal_reserve_districts,region.us.aggregate,06-design-discussion-representation-catalog.md:369
region.us.home_district,region.us.aggregate,06-design-discussion-representation-catalog.md:818-904;07-design-discussion-burrow-composition-probe.md
region.us.metros,region.us.aggregate,06-design-discussion-representation-catalog.md:368
region.us.states,region.us.aggregate,06-design-discussion-representation-catalog.md:368
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/closure/owned_state.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/closure/owned_state.csv
size_bytes: 2723
mode_octal: "0644"
modified_at_utc: 2026-09-04T04:11:43.347265Z
sha256: 90e75c272fabeab540b20081f6148b055bede6d832d4f56b98241ac3764c5c55
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
state_id,owner_id,state_kind,value_domain,unit,conserved,witness_kind,completeness_state,provenance
account.burrow.cash_reserves_collateral,inst.us.bank.burrow,stock,cash; reserves; collateral encumbrance,USD,true,balanced entries; collateral witness; and settlement finality,typed,07-design-discussion-burrow-composition-probe.md:121
account.burrow.deposits,inst.us.bank.burrow,obligation,USD by account owner; demandability; insurance condition,USD,true,balanced account and payment-finality witness,structural,06-design-discussion-representation-catalog.md:830-833;07-design-discussion-burrow-composition-probe.md:119-120
account.burrow.equity,inst.us.bank.burrow,stock,USD book equity; accumulated revaluation and loss accounts,USD,true,balanced accounting and corporate-action witness,structural,06-design-discussion-representation-catalog.md:838-841
account.burrow.funding,inst.us.bank.burrow,obligation,USD secured and unsecured borrowing,USD,true,balanced accounting; collateral; and settlement witness,structural,06-design-discussion-representation-catalog.md:834-837
account.burrow.loans,inst.us.bank.burrow,stock,USD notional; carrying value; market value; exposure,USD,true,balanced accounting and credit-state witness,structural,06-design-discussion-representation-catalog.md:822-825
account.burrow.securities,inst.us.bank.burrow,stock,instrument-family positions; accounting classification; marks,USD,true,position; custody; valuation; and settlement witness,structural,06-design-discussion-representation-catalog.md:826-829;07-design-discussion-burrow-composition-probe.md:117-118
commitment.burrow.remediation,inst.us.bank.burrow,obligation,accepted remediation promises and reserved resources,NONE,false,acknowledgement; performance; breach; expiry; or settlement witness,structural,06-design-discussion-representation-catalog.md:850-853;07-design-discussion-burrow-composition-probe.md:126;07-design-discussion-burrow-composition-probe.md:263
queue.burrow.payment_requests,inst.us.bank.burrow,queue,USD requests with account owner and stable arrival order,USD,false,queue transition and payment-finality witness,structural,06-design-discussion-representation-catalog.md:846-849;07-design-discussion-burrow-composition-probe.md:123
record.burrow.accounting_classification,inst.us.bank.burrow,record,HTM versus AFS classification and carrying-value metadata,NONE,false,classification record plus before/after accounting entries,typed,07-design-discussion-burrow-composition-probe.md:118
state.burrow.operating_capacity,inst.us.bank.burrow,condition,branches; staff; readiness; throughput; limits,NONE,false,operational action-result witness,structural,06-design-discussion-representation-catalog.md:842-845
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/closure/owned_state_transitions.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/closure/owned_state_transitions.csv
size_bytes: 1434
mode_octal: "0644"
modified_at_utc: 2026-09-04T04:04:58.404857Z
sha256: 0c4526cdce3f5aa1eb1115212d0bb26eb2f56aa115db0685b8e3eb99f9c859fa
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
state_id,transition_kind,provenance
account.burrow.cash_reserves_collateral,accounting; collateral pledge; lending transaction; settlement,07-design-discussion-burrow-composition-probe.md:121
account.burrow.deposits,accepted account instruction; settlement,06-design-discussion-representation-catalog.md:830-833
account.burrow.equity,issuance; repurchase; income; revaluation; loss,06-design-discussion-representation-catalog.md:838-841
account.burrow.funding,contract; draw; repayment; revaluation; loss,06-design-discussion-representation-catalog.md:834-837
account.burrow.loans,origination; repayment; transfer; impairment; writeoff,06-design-discussion-representation-catalog.md:822-825
account.burrow.securities,trade; settlement; maturity; pledge; release; loss,06-design-discussion-representation-catalog.md:826-829
commitment.burrow.remediation,commitment creation; revision; performance; breach; expiry; settlement,06-design-discussion-representation-catalog.md:850-853
queue.burrow.payment_requests,acceptance; cancellation; processing; settlement; rejection; failure,06-design-discussion-representation-catalog.md:846-849
record.burrow.accounting_classification,authorized accounting-classification transition; revaluation,07-design-discussion-burrow-composition-probe.md:118
state.burrow.operating_capacity,authorized investment; reservation; damage; repair; release,06-design-discussion-representation-catalog.md:842-845
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/closure/period_variants.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/closure/period_variants.csv
size_bytes: 97
mode_octal: "0644"
modified_at_utc: 2026-09-04T04:06:00.127810Z
sha256: 873f7e67a5057edb4882c201cc7f42d5d05bb0f4b54b08d741c697d39ad3d94c
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
catalog_id,variant_id,effective_period,authority_or_eligibility_ref,uncertainty_notes,provenance
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/closure/probe_coverage.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/closure/probe_coverage.csv
size_bytes: 10881
mode_octal: "0644"
modified_at_utc: 2026-09-04T18:32:41.158234Z
sha256: ad9073a3b72f5ef0cf77d6cbc911fa0e83ca6bbd8460383657ca9b5f59c27cd4
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
probe_id,catalog_id,coverage_status,requirement,uncertainty_notes,provenance
probe.binding_publication,reference.us.nyfed.sofr,covered,Required architecture contract is structurally represented.,Runtime values are assessed separately.,Representation Catalog composition-probe contract.
probe.external_interface_contract,adapter.external.china,covered,interface.us_duration.demand_schedule,Runtime initialization remains outside catalog eligibility.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
probe.external_interface_contract,adapter.external.emerging_markets,covered,interface.foreign_financial_stress.index,Runtime initialization remains outside catalog eligibility.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
probe.external_interface_contract,adapter.external.euro_area,covered,interface.dollar_funding.capacity,Runtime initialization remains outside catalog eligibility.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
probe.external_interface_contract,adapter.external.external_energy,covered,interface.energy_supply.product_schedule,Runtime initialization remains outside catalog eligibility.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
probe.external_interface_contract,adapter.external.external_world,covered,interface.external_demand.index,Runtime initialization remains outside catalog eligibility.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
probe.external_interface_contract,adapter.external.freight_global,covered,interface.freight.capacity,Runtime initialization remains outside catalog eligibility.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
probe.external_interface_contract,adapter.external.freight_gulf_suez,covered,interface.freight.capacity,Runtime initialization remains outside catalog eligibility.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
probe.external_interface_contract,adapter.external.gulf_exporters,covered,interface.energy_supply.product_schedule,Runtime initialization remains outside catalog eligibility.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
probe.external_interface_contract,adapter.external.japan,covered,interface.us_duration.demand_schedule,Runtime initialization remains outside catalog eligibility.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
probe.external_interface_contract,adapter.external.offshore_asia,covered,interface.dollar_funding.capacity,Runtime initialization remains outside catalog eligibility.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
probe.external_interface_contract,adapter.external.oil_exporters,covered,interface.us_duration.demand_schedule,Runtime initialization remains outside catalog eligibility.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
probe.external_interface_contract,adapter.external.russia,covered,interface.energy_supply.product_schedule,Runtime initialization remains outside catalog eligibility.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
probe.external_interface_contract,adapter.external.united_kingdom,covered,interface.dollar_funding.capacity,Runtime initialization remains outside catalog eligibility.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
probe.fallback_parity,cohort.us.bank.regional_cre_concentrated,covered,Required architecture contract is structurally represented.,Runtime values are assessed separately.,Representation Catalog composition-probe contract.
probe.instrument_account_contract,mechanism.us.bank.regional_aggregate,covered,Required architecture contract is structurally represented.,Runtime values are assessed separately.,Representation Catalog composition-probe contract.
probe.instrument_account_contract,mechanism.us.repo.tri_party,covered,Required architecture contract is structurally represented.,Runtime values are assessed separately.,Representation Catalog composition-probe contract.
probe.legal_information_path,type.legal_instrument.deposit_insurance_condition,covered,Required architecture contract is structurally represented.,Runtime values are assessed separately.,Representation Catalog composition-probe contract.
probe.named_residual_replacement,adapter.external.external_world,covered,Required architecture contract is structurally represented.,Runtime values are assessed separately.,Representation Catalog composition-probe contract.
probe.no_magic_price,market.product.global.timber,covered,Required architecture contract is structurally represented.,Runtime values are assessed separately.,Representation Catalog composition-probe contract.
probe.owner_witness_chain,inst.us.bank.burrow,covered,Required architecture contract is structurally represented.,Runtime values are assessed separately.,Representation Catalog composition-probe contract.
probe.product_conservation,process.climate_agriculture.amazon_basin,covered,Required architecture contract is structurally represented.,Runtime values are assessed separately.,Representation Catalog composition-probe contract.
probe.region_process_separation,region.sa.amazon_basin,covered,Required architecture contract is structurally represented.,Runtime values are assessed separately.,Representation Catalog composition-probe contract.
probe.settlement_and_clearing,inst.us.clearing.ficc,covered,Required architecture contract is structurally represented.,Runtime values are assessed separately.,Representation Catalog composition-probe contract.
probe.treasury_basis_graph,market.us.treasury.secondary,covered,Required architecture contract is structurally represented.,Runtime values are assessed separately.,Representation Catalog composition-probe contract.
probe.type_instance_compatibility,inst.us.bank.burrow,covered,Required architecture contract is structurally represented.,Runtime values are assessed separately.,Representation Catalog composition-probe contract.
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/closure/profile_catalog_roles.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/closure/profile_catalog_roles.csv
size_bytes: 105837
mode_octal: "0644"
modified_at_utc: 2026-09-04T18:55:20.042041Z
sha256: ce46ab11eaedebc49244f0e6212756e1538a5e0eb8c627380498ed15066dd002
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
profile_id,catalog_id,profile_role,candidate_provider_entry_id,activation_requirement,rationale,provenance
profile.early_2006.bernankey,adapter.demand.global.coffee,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.demand.global.pork,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.demand.global.timber,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.export.arabian_peninsula.petroleum,reserve,adapter.external.gulf_exporters,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.external.china,boundary_candidate,adapter.external.china,Receiving module selects this provider in a future representation manifest.,Channel-specific external candidate; role does not activate it.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.external.emerging_markets,boundary_candidate,adapter.external.emerging_markets,Receiving module selects this provider in a future representation manifest.,Channel-specific external candidate; role does not activate it.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.external.euro_area,boundary_candidate,adapter.external.euro_area,Receiving module selects this provider in a future representation manifest.,Channel-specific external candidate; role does not activate it.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.external.external_energy,boundary_candidate,adapter.external.external_energy,Receiving module selects this provider in a future representation manifest.,Channel-specific external candidate; role does not activate it.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.external.external_world,boundary_candidate,adapter.external.external_world,Receiving module selects this provider in a future representation manifest.,Channel-specific external candidate; role does not activate it.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.external.freight_global,boundary_candidate,adapter.external.freight_global,Receiving module selects this provider in a future representation manifest.,Channel-specific external candidate; role does not activate it.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.external.freight_gulf_suez,boundary_candidate,adapter.external.freight_gulf_suez,Receiving module selects this provider in a future representation manifest.,Channel-specific external candidate; role does not activate it.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.external.gulf_exporters,boundary_candidate,adapter.external.gulf_exporters,Receiving module selects this provider in a future representation manifest.,Channel-specific external candidate; role does not activate it.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.external.japan,boundary_candidate,adapter.external.japan,Receiving module selects this provider in a future representation manifest.,Channel-specific external candidate; role does not activate it.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.external.offshore_asia,boundary_candidate,adapter.external.offshore_asia,Receiving module selects this provider in a future representation manifest.,Channel-specific external candidate; role does not activate it.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.external.oil_exporters,boundary_candidate,adapter.external.oil_exporters,Receiving module selects this provider in a future representation manifest.,Channel-specific external candidate; role does not activate it.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.external.russia,boundary_candidate,adapter.external.russia,Receiving module selects this provider in a future representation manifest.,Channel-specific external candidate; role does not activate it.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.external.united_kingdom,boundary_candidate,adapter.external.united_kingdom,Receiving module selects this provider in a future representation manifest.,Channel-specific external candidate; role does not activate it.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.flow.global.coffee,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.flow.global.pork,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.flow.global.timber,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.hydropower.caucasus,reserve,adapter.external.emerging_markets,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.hydropower.congo_basin,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.incident.amazon_basin.climate,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.incident.external.chokepoint_blockage,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.incident.external.cyber,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.incident.external.drought,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.incident.external.gaffe_leak,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.incident.external.geological,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.incident.external.outbreak,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.incident.external.statistical_release_failure,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.incident.external.technological_breakthrough,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.incident.external.weather,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.incident.external.wildfire,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.input.amazon_basin.feed_grain,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.input.sa.feed_grain,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.market.global.coffee,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.market.global.pork,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.market.global.timber,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.process.amazon_basin.climate_agriculture,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.process.arabian_peninsula.resources,reserve,adapter.external.gulf_exporters,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.process.caucasus.resources,reserve,adapter.external.emerging_markets,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.process.climate_agriculture,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.process.congo_basin.ecology,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.process.epidemic,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.process.war,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.real.us.construction_inputs,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.real.us.food_baskets,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.real.us.housing_activity,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.sector.arabian_peninsula.gas,reserve,adapter.external.gulf_exporters,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.sector.arabian_peninsula.petroleum,reserve,adapter.external.gulf_exporters,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.sector.caucasus.petroleum,reserve,adapter.external.emerging_markets,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.sector.congo_basin.forestry,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.sector.drc.copper_cobalt,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.supply.row.green_coffee,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.supply.row.live_hogs,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.supply.row.timber,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.transform.coffee_roasting,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.transform.hogs_slaughter,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.transform.timber_milling,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.transit.caucasus.energy,reserve,adapter.external.emerging_markets,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.transport.drc.minerals,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,adapter.water.arabian_peninsula,reserve,adapter.external.gulf_exporters,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,agreement.central_bank.swap_lines,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,agreement.foreign.information_sharing,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,agreement.foreign.liquidity_support,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,agreement.foreign.regulatory_memoranda,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,agreement.foreign.sanctions_coordination,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,agreement.us.federal_reserve.swap_lines,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,agreement.us.repo.bilateral,slice_candidate,agreement.us.repo.bilateral,Bounded repo maturity is active.,Supplies the no-automatic-roll liquidity trigger and explicit obligation settlement.,12-structure-outline-bernankey-mvp-cycle.md:299-319
profile.early_2006.bernankey,auth.us.primary_dealer_designation,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,body.il.cabinet,reserve,adapter.external.emerging_markets,Distinct modeled authority or balance-sheet action; independently researched opening state and history; interface-compatible transmissions; complete named-plus-residual reconciliation.,Thin foreign candidate retained without synthetic regional state.,Early-2006 profile planning index.
profile.early_2006.bernankey,body.jp.cabinet,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,body.opec,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,body.us.federal_reserve.board_voting,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,body.us.federal_reserve.fomc,slice_candidate,NONE,Future manifest selection plus effective-period initialization.,Dependency-closed Treasury-duration or secured-funding candidate.,Early-2006 profile planning index.
profile.early_2006.bernankey,body.us.federal_reserve.reserve_bank_boards,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,body.us.treasury.tbac,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,coalition.il.governing_parties,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,coalition.ir.factional_networks,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,coalition.ru.elite_networks,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,coalition.uk.parties,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,cohort.eu.exposed_banks,reserve,adapter.external.euro_area,Distinct modeled authority or balance-sheet action; independently researched opening state and history; interface-compatible transmissions; complete named-plus-residual reconciliation.,Thin foreign candidate retained without synthetic regional state.,Early-2006 profile planning index.
profile.early_2006.bernankey,cohort.foreign.central_bank,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,cohort.foreign.central_banks,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,cohort.foreign.finance_ministries,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,cohort.foreign.finance_ministry,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,cohort.foreign.sovereign_fund,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,cohort.foreign.sovereign_funds,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,cohort.foreign.state_bank,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,cohort.foreign.state_banks,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,cohort.jp.banks,reserve,adapter.external.japan,Distinct modeled authority or balance-sheet action; independently researched opening state and history; interface-compatible transmissions; complete named-plus-residual reconciliation.,Thin foreign candidate retained without synthetic regional state.,Early-2006 profile planning index.
profile.early_2006.bernankey,cohort.jp.insurers,reserve,adapter.external.japan,Distinct modeled authority or balance-sheet action; independently researched opening state and history; interface-compatible transmissions; complete named-plus-residual reconciliation.,Thin foreign candidate retained without synthetic regional state.,Early-2006 profile planning index.
profile.early_2006.bernankey,cohort.jp.pensions,reserve,adapter.external.japan,Distinct modeled authority or balance-sheet action; independently researched opening state and history; interface-compatible transmissions; complete named-plus-residual reconciliation.,Thin foreign candidate retained without synthetic regional state.,Early-2006 profile planning index.
profile.early_2006.bernankey,cohort.ru.energy_firms,reserve,adapter.external.russia,Distinct modeled authority or balance-sheet action; independently researched opening state and history; interface-compatible transmissions; complete named-plus-residual reconciliation.,Thin foreign candidate retained without synthetic regional state.,Early-2006 profile planning index.
profile.early_2006.bernankey,cohort.sg.dollar_funding_banks,reserve,adapter.external.offshore_asia,Distinct modeled authority or balance-sheet action; independently researched opening state and history; interface-compatible transmissions; complete named-plus-residual reconciliation.,Thin foreign candidate retained without synthetic regional state.,Early-2006 profile planning index.
profile.early_2006.bernankey,cohort.uk.pensions,reserve,adapter.external.united_kingdom,Distinct modeled authority or balance-sheet action; independently researched opening state and history; interface-compatible transmissions; complete named-plus-residual reconciliation.,Thin foreign candidate retained without synthetic regional state.,Early-2006 profile planning index.
profile.early_2006.bernankey,cohort.us.asset_manager,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,cohort.us.bank.community_credit_union,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,cohort.us.bank.regional_cre_concentrated,slice_candidate,NONE,Future manifest selection plus effective-period initialization.,Dependency-closed Treasury-duration or secured-funding candidate.,Early-2006 profile planning index.
profile.early_2006.bernankey,cohort.us.bank.systemically_important,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,cohort.us.custodian.tri_party,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,cohort.us.dealer.non_primary,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,cohort.us.dealer.primary,slice_candidate,cohort.us.dealer.primary,Dealer capacity is active.,Constrains executable demand and owns the dealer accounts.,12-structure-outline-bernankey-mvp-cycle.md:299-319
profile.early_2006.bernankey,cohort.us.fund.money_market_complex,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,cohort.us.fund.small_hedge,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,cohort.us.insurer,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,cohort.us.insurer.regional,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,cohort.us.pension,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,facility.central_bank.swap_lines,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,facility.fima_repo,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,facility.us.federal_reserve.discount_window,slice_candidate,NONE,Future manifest selection plus effective-period initialization.,Dependency-closed Treasury-duration or secured-funding candidate.,Early-2006 profile planning index.
profile.early_2006.bernankey,facility.us.federal_reserve.fima_repo,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,facility.us.federal_reserve.rrp,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,facility.us.federal_reserve.section_13_3,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,facility.us.federal_reserve.srf,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,federated.euro_area,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,federation.us.federal_reserve,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,firm.sa.saudi_aramco,reserve,adapter.external.gulf_exporters,Distinct modeled authority or balance-sheet action; independently researched opening state and history; interface-compatible transmissions; complete named-plus-residual reconciliation.,Thin foreign candidate retained without synthetic regional state.,Early-2006 profile planning index.
profile.early_2006.bernankey,generator.chokepoint_blockage,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,generator.climate.amazon_basin,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,generator.cyber,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,generator.drought,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,generator.gaffe_leak,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,generator.geological,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,generator.outbreak,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,generator.statistical_release_failure,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,generator.technological_breakthrough,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,generator.weather,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,generator.wildfire,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,household.us.cohorts,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,industry.amazon_basin.coffee,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,industry.amazon_basin.forestry,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,industry.amazon_basin.pork,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,industry.arabian_peninsula.gas,reserve,adapter.external.gulf_exporters,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,industry.arabian_peninsula.petroleum,reserve,adapter.external.gulf_exporters,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,industry.caucasus.petroleum,reserve,adapter.external.emerging_markets,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,industry.congo_basin.forestry,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,industry.drc.copper_cobalt,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,industry.sa.coffee.aggregate,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,industry.sa.forestry.aggregate,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,industry.sa.pork.aggregate,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,inst.global.bis,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,inst.global.imf,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,inst.global.settlement.cls,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,inst.us.bank.burrow,slice_candidate,NONE,Future manifest selection plus effective-period initialization.,Dependency-closed Treasury-duration or secured-funding candidate.,Early-2006 profile planning index.
profile.early_2006.bernankey,inst.us.clearing.cme,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,inst.us.clearing.ficc,slice_candidate,NONE,Future manifest selection plus effective-period initialization.,Dependency-closed Treasury-duration or secured-funding candidate.,Early-2006 profile planning index.
profile.early_2006.bernankey,inst.us.cme,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,inst.us.depository.dtcc,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,inst.us.federal_reserve.board,slice_candidate,NONE,Future manifest selection plus effective-period initialization.,Dependency-closed Treasury-duration or secured-funding candidate.,Early-2006 profile planning index.
profile.early_2006.bernankey,inst.us.federal_reserve.new_york,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,inst.us.federal_reserve.reserve_banks_other,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,inst.us.fhlb.system,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,inst.us.ficc,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,inst.us.fund.macro_fund_7,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,inst.us.fund.pamplona_brothers,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,inst.us.gse.fannie_mae,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,inst.us.gse.freddie_mac,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,inst.us.leveraged_funds,slice_candidate,inst.us.leveraged_funds,Repo borrower is active.,Converts the witnessed liquidity deficit into a bounded Treasury sale.,12-structure-outline-bernankey-mvp-cycle.md:299-319
profile.early_2006.bernankey,inst.us.primary_dealers,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,inst.us.systemically_important_banks,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,inst.us.treasury,slice_candidate,NONE,Future manifest selection plus effective-period initialization.,Dependency-closed Treasury-duration or secured-funding candidate.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.ae.central_bank,reserve,adapter.external.gulf_exporters,Distinct modeled authority or balance-sheet action; independently researched opening state and history; interface-compatible transmissions; complete named-plus-residual reconciliation.,Thin foreign candidate retained without synthetic regional state.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.ae.energy_companies,reserve,adapter.external.gulf_exporters,Distinct modeled authority or balance-sheet action; independently researched opening state and history; interface-compatible transmissions; complete named-plus-residual reconciliation.,Thin foreign candidate retained without synthetic regional state.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.ae.sovereign_funds,reserve,adapter.external.gulf_exporters,Distinct modeled authority or balance-sheet action; independently researched opening state and history; interface-compatible transmissions; complete named-plus-residual reconciliation.,Thin foreign candidate retained without synthetic regional state.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.bis,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.cn.military,reserve,adapter.external.china,Distinct modeled authority or balance-sheet action; independently researched opening state and history; interface-compatible transmissions; complete named-plus-residual reconciliation.,Thin foreign candidate retained without synthetic regional state.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.cn.party_leadership,reserve,adapter.external.china,Distinct modeled authority or balance-sheet action; independently researched opening state and history; interface-compatible transmissions; complete named-plus-residual reconciliation.,Thin foreign candidate retained without synthetic regional state.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.cn.pboc,reserve,adapter.external.china,Distinct modeled authority or balance-sheet action; independently researched opening state and history; interface-compatible transmissions; complete named-plus-residual reconciliation.,Thin foreign candidate retained without synthetic regional state.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.cn.policy_banks,reserve,adapter.external.china,Distinct modeled authority or balance-sheet action; independently researched opening state and history; interface-compatible transmissions; complete named-plus-residual reconciliation.,Thin foreign candidate retained without synthetic regional state.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.cn.safe,reserve,adapter.external.china,Distinct modeled authority or balance-sheet action; independently researched opening state and history; interface-compatible transmissions; complete named-plus-residual reconciliation.,Thin foreign candidate retained without synthetic regional state.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.cn.state_council,reserve,adapter.external.china,Distinct modeled authority or balance-sheet action; independently researched opening state and history; interface-compatible transmissions; complete named-plus-residual reconciliation.,Thin foreign candidate retained without synthetic regional state.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.de.government,reserve,adapter.external.euro_area,Distinct modeled authority or balance-sheet action; independently researched opening state and history; interface-compatible transmissions; complete named-plus-residual reconciliation.,Thin foreign candidate retained without synthetic regional state.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.eu.commission,reserve,adapter.external.euro_area,Distinct modeled authority or balance-sheet action; independently researched opening state and history; interface-compatible transmissions; complete named-plus-residual reconciliation.,Thin foreign candidate retained without synthetic regional state.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.eu.ecb,reserve,adapter.external.euro_area,Distinct modeled authority or balance-sheet action; independently researched opening state and history; interface-compatible transmissions; complete named-plus-residual reconciliation.,Thin foreign candidate retained without synthetic regional state.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.eu.member_governments,reserve,adapter.external.euro_area,Distinct modeled authority or balance-sheet action; independently researched opening state and history; interface-compatible transmissions; complete named-plus-residual reconciliation.,Thin foreign candidate retained without synthetic regional state.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.eu.national_central_banks,reserve,adapter.external.euro_area,Distinct modeled authority or balance-sheet action; independently researched opening state and history; interface-compatible transmissions; complete named-plus-residual reconciliation.,Thin foreign candidate retained without synthetic regional state.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.fr.government,reserve,adapter.external.euro_area,Distinct modeled authority or balance-sheet action; independently researched opening state and history; interface-compatible transmissions; complete named-plus-residual reconciliation.,Thin foreign candidate retained without synthetic regional state.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.il.bank_of_israel,reserve,adapter.external.emerging_markets,Distinct modeled authority or balance-sheet action; independently researched opening state and history; interface-compatible transmissions; complete named-plus-residual reconciliation.,Thin foreign candidate retained without synthetic regional state.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.il.security,reserve,adapter.external.emerging_markets,Distinct modeled authority or balance-sheet action; independently researched opening state and history; interface-compatible transmissions; complete named-plus-residual reconciliation.,Thin foreign candidate retained without synthetic regional state.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.imf,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.ir.central_bank,reserve,adapter.external.emerging_markets,Distinct modeled authority or balance-sheet action; independently researched opening state and history; interface-compatible transmissions; complete named-plus-residual reconciliation.,Thin foreign candidate retained without synthetic regional state.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.ir.irgc,reserve,adapter.external.emerging_markets,Distinct modeled authority or balance-sheet action; independently researched opening state and history; interface-compatible transmissions; complete named-plus-residual reconciliation.,Thin foreign candidate retained without synthetic regional state.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.ir.oil,reserve,adapter.external.emerging_markets,Distinct modeled authority or balance-sheet action; independently researched opening state and history; interface-compatible transmissions; complete named-plus-residual reconciliation.,Thin foreign candidate retained without synthetic regional state.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.it.government,reserve,adapter.external.euro_area,Distinct modeled authority or balance-sheet action; independently researched opening state and history; interface-compatible transmissions; complete named-plus-residual reconciliation.,Thin foreign candidate retained without synthetic regional state.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.jp.boj,reserve,adapter.external.japan,Distinct modeled authority or balance-sheet action; independently researched opening state and history; interface-compatible transmissions; complete named-plus-residual reconciliation.,Thin foreign candidate retained without synthetic regional state.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.jp.mof,reserve,adapter.external.japan,Distinct modeled authority or balance-sheet action; independently researched opening state and history; interface-compatible transmissions; complete named-plus-residual reconciliation.,Thin foreign candidate retained without synthetic regional state.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.media.aftv,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.media.gnbc,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.media.honkbox,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.media.loonberg,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.media.wool_street_journal,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.qa.central_bank,reserve,adapter.external.gulf_exporters,Distinct modeled authority or balance-sheet action; independently researched opening state and history; interface-compatible transmissions; complete named-plus-residual reconciliation.,Thin foreign candidate retained without synthetic regional state.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.qa.energy_companies,reserve,adapter.external.gulf_exporters,Distinct modeled authority or balance-sheet action; independently researched opening state and history; interface-compatible transmissions; complete named-plus-residual reconciliation.,Thin foreign candidate retained without synthetic regional state.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.qa.sovereign_funds,reserve,adapter.external.gulf_exporters,Distinct modeled authority or balance-sheet action; independently researched opening state and history; interface-compatible transmissions; complete named-plus-residual reconciliation.,Thin foreign candidate retained without synthetic regional state.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.ru.central_bank,reserve,adapter.external.russia,Distinct modeled authority or balance-sheet action; independently researched opening state and history; interface-compatible transmissions; complete named-plus-residual reconciliation.,Thin foreign candidate retained without synthetic regional state.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.ru.presidency,reserve,adapter.external.russia,Distinct modeled authority or balance-sheet action; independently researched opening state and history; interface-compatible transmissions; complete named-plus-residual reconciliation.,Thin foreign candidate retained without synthetic regional state.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.ru.security,reserve,adapter.external.russia,Distinct modeled authority or balance-sheet action; independently researched opening state and history; interface-compatible transmissions; complete named-plus-residual reconciliation.,Thin foreign candidate retained without synthetic regional state.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.sa.energy_ministry,reserve,adapter.external.gulf_exporters,Distinct modeled authority or balance-sheet action; independently researched opening state and history; interface-compatible transmissions; complete named-plus-residual reconciliation.,Thin foreign candidate retained without synthetic regional state.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.sa.sama,reserve,adapter.external.gulf_exporters,Distinct modeled authority or balance-sheet action; independently researched opening state and history; interface-compatible transmissions; complete named-plus-residual reconciliation.,Thin foreign candidate retained without synthetic regional state.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.sa.sovereign_funds,reserve,adapter.external.gulf_exporters,Distinct modeled authority or balance-sheet action; independently researched opening state and history; interface-compatible transmissions; complete named-plus-residual reconciliation.,Thin foreign candidate retained without synthetic regional state.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.sg.mas,reserve,adapter.external.offshore_asia,Distinct modeled authority or balance-sheet action; independently researched opening state and history; interface-compatible transmissions; complete named-plus-residual reconciliation.,Thin foreign candidate retained without synthetic regional state.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.sg.shipping,reserve,adapter.external.offshore_asia,Distinct modeled authority or balance-sheet action; independently researched opening state and history; interface-compatible transmissions; complete named-plus-residual reconciliation.,Thin foreign candidate retained without synthetic regional state.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.sg.sovereign_funds,reserve,adapter.external.offshore_asia,Distinct modeled authority or balance-sheet action; independently researched opening state and history; interface-compatible transmissions; complete named-plus-residual reconciliation.,Thin foreign candidate retained without synthetic regional state.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.uk.boe,reserve,adapter.external.united_kingdom,Distinct modeled authority or balance-sheet action; independently researched opening state and history; interface-compatible transmissions; complete named-plus-residual reconciliation.,Thin foreign candidate retained without synthetic regional state.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.uk.dmo,reserve,adapter.external.united_kingdom,Distinct modeled authority or balance-sheet action; independently researched opening state and history; interface-compatible transmissions; complete named-plus-residual reconciliation.,Thin foreign candidate retained without synthetic regional state.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.uk.government,reserve,adapter.external.united_kingdom,Distinct modeled authority or balance-sheet action; independently researched opening state and history; interface-compatible transmissions; complete named-plus-residual reconciliation.,Thin foreign candidate retained without synthetic regional state.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.us.american.bankers.association,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.us.bank.policy.institute,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.us.bea,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.us.bls,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.us.chamber.of.commerce,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.us.community.reinvestment.advocates,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.us.consumer.finance.advocates,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.us.farm.bureau.analogue,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.us.homebuilders,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.us.independent.community.bankers,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.us.investment.company.institute.analogue,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.us.labor.federation,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.us.managed.funds.association,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.us.member.unions,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.us.mortgage.bankers,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.us.new_york_fed,slice_candidate,NONE,Future manifest selection plus effective-period initialization.,Dependency-closed Treasury-duration or secured-funding candidate.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.us.policy.institutes,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.us.realtors,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.us.retiree.association.aarp.analogue,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.us.securities.industry.association,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.us.small.business.federation,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,institution.us.think.tanks,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,instrument.cross_border.capital_controls,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,legal.us.dodd_frank,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,legal.us.federal_reserve.section_13_3,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,market.global.fx_spot_forward_basis,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,market.product.global.coffee,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,market.product.global.pork,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,market.product.global.timber,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,market.us.corporate_bond,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,market.us.equity,slice_candidate,NONE,Future manifest selection plus effective-period initialization.,Dependency-closed Treasury-duration or secured-funding candidate.,Early-2006 profile planning index.
profile.early_2006.bernankey,market.us.fed_funds,slice_candidate,NONE,Future manifest selection plus effective-period initialization.,Dependency-closed Treasury-duration or secured-funding candidate.,Early-2006 profile planning index.
profile.early_2006.bernankey,market.us.mbs_tba,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,market.us.sofr_futures,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,market.us.treasury.auction,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,market.us.treasury.futures,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,market.us.treasury.secondary,slice_candidate,market.us.treasury.secondary,5-10 year Treasury bucket is active.,Owns endogenous price allocation rationing and clearing failure.,12-structure-outline-bernankey-mvp-cycle.md:299-319
profile.early_2006.bernankey,market.us.treasury_futures,slice_candidate,NONE,Future manifest selection plus effective-period initialization.,Dependency-closed Treasury-duration or secured-funding candidate.,Early-2006 profile planning index.
profile.early_2006.bernankey,mechanism.cross_border.chokepoint_shipping,reserve,adapter.external.emerging_markets,Distinct modeled authority or balance-sheet action; independently researched opening state and history; interface-compatible transmissions; complete named-plus-residual reconciliation.,Thin foreign candidate retained without synthetic regional state.,Early-2006 profile planning index.
profile.early_2006.bernankey,mechanism.demand.global.coffee,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,mechanism.demand.global.pork,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,mechanism.demand.global.timber,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,mechanism.export.arabian_peninsula.petroleum,reserve,adapter.external.gulf_exporters,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,mechanism.flow.global.coffee,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,mechanism.flow.global.pork,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,mechanism.flow.global.timber,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,mechanism.hydropower.caucasus,reserve,adapter.external.emerging_markets,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,mechanism.hydropower.congo_basin,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,mechanism.measurement.us.consumer_prices,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,mechanism.sector.sa.coffee,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,mechanism.sector.sa.forestry,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,mechanism.sector.sa.hog_production,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,mechanism.transform.global.coffee_roasting,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,mechanism.transform.global.hogs_slaughter,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,mechanism.transform.global.timber_milling,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,mechanism.transit.caucasus.energy,reserve,adapter.external.emerging_markets,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,mechanism.transport.drc.minerals,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,mechanism.us.bank.regional_aggregate,slice_candidate,NONE,Future manifest selection plus effective-period initialization.,Dependency-closed Treasury-duration or secured-funding candidate.,Early-2006 profile planning index.
profile.early_2006.bernankey,mechanism.us.chips,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,mechanism.us.clearing.ficc_repo,slice_candidate,NONE,Future manifest selection plus effective-period initialization.,Dependency-closed Treasury-duration or secured-funding candidate.,Early-2006 profile planning index.
profile.early_2006.bernankey,mechanism.us.external_market_inputs,slice_candidate,NONE,Future manifest selection plus effective-period initialization.,Dependency-closed Treasury-duration or secured-funding candidate.,Early-2006 profile planning index.
profile.early_2006.bernankey,mechanism.us.fedwire.funds,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,mechanism.us.fedwire.securities,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,mechanism.us.ficc.sponsored_gcf_repo,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,mechanism.us.nss,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,mechanism.us.payment.chips,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,mechanism.us.payment.fedwire_funds,slice_candidate,NONE,Future manifest selection plus effective-period initialization.,Dependency-closed Treasury-duration or secured-funding candidate.,Early-2006 profile planning index.
profile.early_2006.bernankey,mechanism.us.repo.tri_party,slice_candidate,NONE,Future manifest selection plus effective-period initialization.,Dependency-closed Treasury-duration or secured-funding candidate.,Early-2006 profile planning index.
profile.early_2006.bernankey,mechanism.us.settlement.fedwire_securities,slice_candidate,NONE,Future manifest selection plus effective-period initialization.,Dependency-closed Treasury-duration or secured-funding candidate.,Early-2006 profile planning index.
profile.early_2006.bernankey,mechanism.us.settlement.nss,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,mechanism.water.arabian_peninsula,reserve,adapter.external.gulf_exporters,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,network.media.goosetogetherstrong,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,network.media.honkbox,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,network.media.the_herd,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,office.media.aftv.anchor_chair,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,office.media.gnbc.anchor_chair,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,office.media.wool_street_journal.fed_beat,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,office.us.federal_reserve.board_chair,anchor,NONE,NONE,Player identity or required Chair office for the early-2006 profile.,Early-2006 profile planning index.
profile.early_2006.bernankey,office.us.federal_reserve.chief_of_staff,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,office.us.federal_reserve.division_director,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,office.us.federal_reserve.fomc_chair,anchor,NONE,NONE,Player identity or required Chair office for the early-2006 profile.,Early-2006 profile planning index.
profile.early_2006.bernankey,office.us.federal_reserve.governor,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,office.us.federal_reserve.ny_president,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,office.us.federal_reserve.reserve_bank_president,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,office.us.federal_reserve.vice_chair,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,office.us.federal_reserve.vice_chair_supervision,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,office.us.treasury.secretary,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,organization.us.agricultural.producers,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,organization.us.all.banks,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,organization.us.asset.managers,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,organization.us.business.generally,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,organization.us.community.banks.credit.unions,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,organization.us.dealers.market.makers,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,organization.us.endowments,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,organization.us.hedge.funds,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,organization.us.homebuilders,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,organization.us.insurers,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,organization.us.large.banks,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,organization.us.money.market.fund.complexes,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,organization.us.money.market.mutual.funds,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,organization.us.mortgage.originators,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,organization.us.municipalities.by.rating.market.access,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,organization.us.nonprimary.broker.dealers,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,organization.us.nonprofits,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,organization.us.payroll.intermediaries,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,organization.us.pensions,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,organization.us.real.estate.agents,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,organization.us.regional.insurers,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,organization.us.small.businesses,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,organization.us.small.firms,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,organization.us.small.hedge.funds,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,organization_cohort.market.credit_rating_agencies,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,outlet.media.aftv,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,outlet.media.gnbc,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,outlet.media.loonberg,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,outlet.media.wool_street_journal,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,person.cn.pboc_governor,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,person.cn.safe_head,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,person.cn.xi,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,person.eu.ecb_president,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,person.il.boi_governor,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,person.il.prime_minister,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,person.ir.central_bank_head,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,person.ir.irgc_head,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,person.ir.president,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,person.ir.supreme_leader,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,person.jp.boj_governor,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,person.jp.mof_international_affairs_vice_minister,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,person.media.aftv_host,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,person.media.gnbc_anchor,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,person.media.wool_street_journal_fed_reporter,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,person.sa.aramco_ceo,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,person.sa.crown,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,person.sa.pif_head,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,person.sa.sama_governor,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,person.uk.boe_governor,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,person.uk.dmo_chief,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,person.us.alan_greenspaniel,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,person.us.ben_bernankey,anchor,NONE,NONE,Player identity or required Chair office for the early-2006 profile.,Early-2006 profile planning index.
profile.early_2006.bernankey,person.us.janet_jackrabbit,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,person.us.jerome_owl,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,person.us.kevin_boarsh,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,person.us.paul_vulture,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,person.us.primary_dealer.rates_head,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,person.us.treasury.secretary,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,pop.us.affluent.homeowners,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,pop.us.college.students,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,pop.us.fixed.rate.homeowners.by.mortgage.vintage,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,pop.us.fixed.rate.owners.low.mortgage.vintages,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,pop.us.high.income.finance.professionals,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,pop.us.high.income.professionals,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,pop.us.leftist.male.college.students.georgia.tech,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,pop.us.low.income.service.workers,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,pop.us.low.income.underserved.borrowers,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,pop.us.near.retirees,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,pop.us.parents.paying.childcare,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,pop.us.renters.supply.constrained.metros,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,pop.us.retail.traders,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,pop.us.retirees,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,pop.us.small.business.owners,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,pop.us.strategic.production.workers.fab.technicians,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,pop.us.suburban.parents.childcare.costs,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,pop.us.uninsured.depositors,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,pop.us.workers.by.sector,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,pop.us.young.renters,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,poplens.media.honkbox.posting_population,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,population.us.person.cells,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,process.climate_agriculture.amazon_basin,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,process.climate_agriculture.regional,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,process.ecology.congo_basin,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,process.epidemic,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,process.resources.arabian_peninsula,reserve,adapter.external.gulf_exporters,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,process.resources.caucasus,reserve,adapter.external.emerging_markets,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,process.us.statistical_release_calendar,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,process.war,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,record.media.goosetogetherstrong.margin_call,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,record.media.loonberg.ust_10y_auction_tail,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,record.media.the_herd.large_fox_shop_cutting_10s,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,record.us.federal_reserve.analytical_task,slice_candidate,record.us.federal_reserve.analytical_task,Chair requests the selected Markets comparison.,Preserves request assignment displacement deadline and result as separate witnessed stages.,12-structure-outline-bernankey-mvp-cycle.md:388-457
profile.early_2006.bernankey,record.us.federal_reserve.assessment,slice_candidate,record.us.federal_reserve.assessment,Accepted Markets work completes before the meeting deadline.,Preserves inference evidence uncertainty dissent delivery and later revision.,12-structure-outline-bernankey-mvp-cycle.md:388-457
profile.early_2006.bernankey,record.us.federal_reserve.case_file,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,record.us.federal_reserve.chairmanship_program,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,record.us.federal_reserve.discount_rate,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,record.us.federal_reserve.institutional_project,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,record.us.federal_reserve.iorb,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,record.us.federal_reserve.legacy_dossier,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,record.us.federal_reserve.policy_package,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,record.us.federal_reserve.target_range,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,record.us.treasury.esf,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,record.us.treasury.indemnification_13_3,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,reference.market.credit_ratings,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,reference.market.index_membership,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,reference.market.move,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,reference.market.vendor_indices,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,reference.market.vix,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,reference.unknown.move_index,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,reference.unknown.vix,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,reference.us.bea.pce,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,reference.us.bls.cpi,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,reference.us.consumer_price_index,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,reference.us.federal_reserve.acm_term_premium,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,reference.us.federal_reserve.effr,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,reference.us.federal_reserve.obfr,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,reference.us.nyfed.effr,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,reference.us.nyfed.obfr,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,reference.us.nyfed.sofr,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,reference.us.personal_consumption_expenditures,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,reference.us.treasury.par_yield_curve,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,region.af.congo_basin,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,region.cn.aggregate,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,region.cn.provinces,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,region.eurasia.caucasus,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,region.external.china,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,region.external.emerging_markets,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,region.external.euro_area,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,region.external.external_energy,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,region.external.external_world,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,region.external.freight_global,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,region.external.freight_gulf_suez,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,region.external.gulf_exporters,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,region.external.japan,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,region.external.offshore_asia,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,region.external.oil_exporters,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,region.external.russia,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,region.external.united_kingdom,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,region.hormuz,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,region.me.arabian_peninsula,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,region.middle_east.gulf,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,region.panama,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,region.sa.aggregate,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,region.sa.amazon_basin,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,region.suez,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,region.taiwan_strait,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,region.us.aggregate,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,region.us.census_divisions,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,region.us.federal_reserve_districts,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,region.us.home_district,reference,NONE,NONE,Non-owning scope used by the Burrow composition contract.,Early-2006 profile planning index.
profile.early_2006.bernankey,region.us.metros,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,region.us.states,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,schedule.us.federal_reserve.blackout,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,schedule.us.federal_reserve.fomc,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,schedule.us.treasury.auction,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,scheduled_process.unknown.call_report_calendar,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,sovereign.armenia,reserve,adapter.external.emerging_markets,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,sovereign.azerbaijan,reserve,adapter.external.emerging_markets,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,sovereign.china,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,sovereign.drc,reserve,adapter.external.external_world,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,sovereign.france,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,sovereign.georgia,reserve,adapter.external.emerging_markets,"Source-backed opening state, covered probes, and named-plus-residual reconciliation.",Rich regional content is reserved for composition and future promotion.,Early-2006 profile planning index.
profile.early_2006.bernankey,sovereign.germany,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,sovereign.iran,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,sovereign.israel,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,sovereign.italy,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,sovereign.japan,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,sovereign.qatar,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,sovereign.russia,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,sovereign.saudi_arabia,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,sovereign.singapore,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,sovereign.uae,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,sovereign.united_kingdom,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,staff.us.federal_reserve.communications,slice_candidate,staff.us.federal_reserve.communications,The bounded FOMC cycle initializes Communications access methods and capacity.,Retains the selected staff owner before Phase 5 publication work.,12-structure-outline-bernankey-mvp-cycle.md:397-415
profile.early_2006.bernankey,staff.us.federal_reserve.financial_stability,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,staff.us.federal_reserve.international,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,staff.us.federal_reserve.legal,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,staff.us.federal_reserve.markets,slice_candidate,staff.us.federal_reserve.markets,Chair requests the selected dealer-capacity comparison.,Owns scoped evidence analysis capacity displaced work and the assessment.,12-structure-outline-bernankey-mvp-cycle.md:388-457
profile.early_2006.bernankey,staff.us.federal_reserve.monetary_affairs,slice_candidate,staff.us.federal_reserve.monetary_affairs,The meeting packet carries a named policy-method dissent.,Keeps disagreement attributable to a separate staff unit without asserting canonical truth.,12-structure-outline-bernankey-mvp-cycle.md:388-457
profile.early_2006.bernankey,staff.us.federal_reserve.ny_markets_group,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,staff.us.federal_reserve.research,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,staff.us.federal_reserve.supervision,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,staff.us.treasury.debt_management,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,staff.us.treasury.ofr,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
profile.early_2006.bernankey,stateful.us.treasury.duration_supply,reference,NONE,NONE,Typed identity or non-causal scope retained for reference.,Early-2006 profile planning index.
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/closure/relationships.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/closure/relationships.csv
size_bytes: 39159
mode_octal: "0644"
modified_at_utc: 2026-09-04T04:17:49.345613Z
sha256: 511f017d3d313a0f176ba0c6e278db2dec08e9771a5c5a65a1efb2f1dec454a6
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
relationship_id,relationship_family,subject_entry_id,object_entry_id,effective_period,canonical_owner_id,observability,lifecycle_and_exit,witness_kind,uncertainty_notes,provenance
rel.burrow.operates_in_home_district,OPERATES_IN,inst.us.bank.burrow,region.us.home_district,2006-02-01/2006-12-31,inst.us.bank.burrow,scoped,Ends when the bank footprint or profile scope changes.,charter and branch-footprint record,Exact district selected by a future manifest.,06-design-discussion-representation-catalog.md:818-904;07-design-discussion-burrow-composition-probe.md
rel.holds.ben_bernankey.board_chair,HOLDS,person.us.ben_bernankey,office.us.federal_reserve.board_chair,2006-02-01/2006-12-31,person.us.ben_bernankey,public,Effective during the early-2006 profile; later officeholders require a separate profile relationship.,official chair chronology,NONE,Federal Reserve History: https://www.federalreservehistory.org/people/federal-reserve-chair
rel.holds.ben_bernankey.fomc_chair,HOLDS,person.us.ben_bernankey,office.us.federal_reserve.fomc_chair,2006-02-01/2006-12-31,person.us.ben_bernankey,public,Effective during the early-2006 profile; later officeholders require a separate profile relationship.,official chair chronology,NONE,Federal Reserve History: https://www.federalreservehistory.org/people/federal-reserve-chair
rel.operates_in_scope.sa.energy_ministry.arabian_peninsula,operates_in_scope,institution.sa.energy_ministry,region.me.arabian_peninsula,scenario.gulf_oil_dollar_peg,institution.sa.energy_ministry,scenario initialization witness,Active only for the selected scenario boundary version.,scenario initialization and boundary-version witness,The regional scope and production system do not acquire firm state or action ownership.,04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.operates_in_scope.sa.saudi_aramco.arabian_peninsula,operates_in_scope,firm.sa.saudi_aramco,region.me.arabian_peninsula,scenario.gulf_oil_dollar_peg,firm.sa.saudi_aramco,scenario initialization witness,Active only for the selected scenario boundary version.,scenario initialization and boundary-version witness,The regional scope and production system do not acquire firm state or action ownership.,04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.part_of.us.home_district.us_aggregate,PART_OF,region.us.home_district,region.us.aggregate,2006-02-01/2006-12-31,region.us.home_district,public,Versioned with the profile scope binding.,scenario initialization scope witness,Exact district selected by a future manifest.,06-design-discussion-representation-catalog.md:818-904;07-design-discussion-burrow-composition-probe.md
rel.produces_through.sa.saudi_aramco.arabian_peninsula_petroleum,produces_through,firm.sa.saudi_aramco,industry.arabian_peninsula.petroleum,scenario.gulf_oil_dollar_peg,firm.sa.saudi_aramco,scenario initialization witness,Active only for the selected scenario boundary version.,scenario initialization and boundary-version witness,The regional scope and production system do not acquire firm state or action ownership.,04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.represents_scope.china,REPRESENTS_SCOPE,adapter.external.china,region.external.china,2006-02-01/2006-12-31,adapter.external.china,public,Candidate profile-period scope binding; removal does not alter scope identity.,official classification source,This relationship grants no ownership or authority.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
rel.represents_scope.emerging_markets,REPRESENTS_SCOPE,adapter.external.emerging_markets,region.external.emerging_markets,2006-02-01/2006-12-31,adapter.external.emerging_markets,public,Candidate profile-period scope binding; removal does not alter scope identity.,official classification source,This relationship grants no ownership or authority.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
rel.represents_scope.euro_area,REPRESENTS_SCOPE,adapter.external.euro_area,region.external.euro_area,2006-02-01/2006-12-31,adapter.external.euro_area,public,Candidate profile-period scope binding; removal does not alter scope identity.,official classification source,This relationship grants no ownership or authority.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
rel.represents_scope.external_energy,REPRESENTS_SCOPE,adapter.external.external_energy,region.external.external_energy,2006-02-01/2006-12-31,adapter.external.external_energy,public,Candidate profile-period scope binding; removal does not alter scope identity.,official classification source,This relationship grants no ownership or authority.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
rel.represents_scope.external_world,REPRESENTS_SCOPE,adapter.external.external_world,region.external.external_world,2006-02-01/2006-12-31,adapter.external.external_world,public,Candidate profile-period scope binding; removal does not alter scope identity.,official classification source,This relationship grants no ownership or authority.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
rel.represents_scope.freight_global,REPRESENTS_SCOPE,adapter.external.freight_global,region.external.freight_global,2006-02-01/2006-12-31,adapter.external.freight_global,public,Candidate profile-period scope binding; removal does not alter scope identity.,official classification source,This relationship grants no ownership or authority.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
rel.represents_scope.freight_gulf_suez,REPRESENTS_SCOPE,adapter.external.freight_gulf_suez,region.external.freight_gulf_suez,2006-02-01/2006-12-31,adapter.external.freight_gulf_suez,public,Candidate profile-period scope binding; removal does not alter scope identity.,official classification source,This relationship grants no ownership or authority.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
rel.represents_scope.gulf_exporters,REPRESENTS_SCOPE,adapter.external.gulf_exporters,region.external.gulf_exporters,2006-02-01/2006-12-31,adapter.external.gulf_exporters,public,Candidate profile-period scope binding; removal does not alter scope identity.,official classification source,This relationship grants no ownership or authority.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
rel.represents_scope.japan,REPRESENTS_SCOPE,adapter.external.japan,region.external.japan,2006-02-01/2006-12-31,adapter.external.japan,public,Candidate profile-period scope binding; removal does not alter scope identity.,official classification source,This relationship grants no ownership or authority.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
rel.represents_scope.offshore_asia,REPRESENTS_SCOPE,adapter.external.offshore_asia,region.external.offshore_asia,2006-02-01/2006-12-31,adapter.external.offshore_asia,public,Candidate profile-period scope binding; removal does not alter scope identity.,official classification source,This relationship grants no ownership or authority.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
rel.represents_scope.oil_exporters,REPRESENTS_SCOPE,adapter.external.oil_exporters,region.external.oil_exporters,2006-02-01/2006-12-31,adapter.external.oil_exporters,public,Candidate profile-period scope binding; removal does not alter scope identity.,official classification source,This relationship grants no ownership or authority.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
rel.represents_scope.russia,REPRESENTS_SCOPE,adapter.external.russia,region.external.russia,2006-02-01/2006-12-31,adapter.external.russia,public,Candidate profile-period scope binding; removal does not alter scope identity.,official classification source,This relationship grants no ownership or authority.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
rel.represents_scope.united_kingdom,REPRESENTS_SCOPE,adapter.external.united_kingdom,region.external.united_kingdom,2006-02-01/2006-12-31,adapter.external.united_kingdom,public,Candidate profile-period scope binding; removal does not alter scope identity.,official classification source,This relationship grants no ownership or authority.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
rel.scoped_to.ae.central_bank.uae,scoped_to,institution.ae.central_bank,sovereign.uae,scenario.gulf_oil_dollar_peg,sovereign.uae,scenario initialization witness,Active only while the selected scenario manifest promotes this boundary owner.,scenario initialization and boundary-version witness,"Scope does not transfer state, action, or capacity ownership.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.scoped_to.ae.energy_companies.uae,scoped_to,institution.ae.energy_companies,sovereign.uae,scenario.gulf_oil_dollar_peg,sovereign.uae,scenario initialization witness,Active only while the selected scenario manifest promotes this boundary owner.,scenario initialization and boundary-version witness,"Scope does not transfer state, action, or capacity ownership.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.scoped_to.ae.sovereign_funds.uae,scoped_to,institution.ae.sovereign_funds,sovereign.uae,scenario.gulf_oil_dollar_peg,sovereign.uae,scenario initialization witness,Active only while the selected scenario manifest promotes this boundary owner.,scenario initialization and boundary-version witness,"Scope does not transfer state, action, or capacity ownership.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.scoped_to.cn.military.china,scoped_to,institution.cn.military,sovereign.china,scenario.china_trade_reserve_stress,sovereign.china,scenario initialization witness,Active only while the selected scenario manifest promotes this boundary owner.,scenario initialization and boundary-version witness,"Scope does not transfer state, action, or capacity ownership.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.scoped_to.cn.party_leadership.china,scoped_to,institution.cn.party_leadership,sovereign.china,scenario.china_trade_reserve_stress,sovereign.china,scenario initialization witness,Active only while the selected scenario manifest promotes this boundary owner.,scenario initialization and boundary-version witness,"Scope does not transfer state, action, or capacity ownership.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.scoped_to.cn.pboc.china,scoped_to,institution.cn.pboc,sovereign.china,scenario.china_trade_reserve_stress,sovereign.china,scenario initialization witness,Active only while the selected scenario manifest promotes this boundary owner.,scenario initialization and boundary-version witness,"Scope does not transfer state, action, or capacity ownership.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.scoped_to.cn.policy_banks.china,scoped_to,institution.cn.policy_banks,sovereign.china,scenario.china_trade_reserve_stress,sovereign.china,scenario initialization witness,Active only while the selected scenario manifest promotes this boundary owner.,scenario initialization and boundary-version witness,"Scope does not transfer state, action, or capacity ownership.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.scoped_to.cn.safe.china,scoped_to,institution.cn.safe,sovereign.china,scenario.china_trade_reserve_stress,sovereign.china,scenario initialization witness,Active only while the selected scenario manifest promotes this boundary owner.,scenario initialization and boundary-version witness,"Scope does not transfer state, action, or capacity ownership.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.scoped_to.cn.state_council.china,scoped_to,institution.cn.state_council,sovereign.china,scenario.china_trade_reserve_stress,sovereign.china,scenario initialization witness,Active only while the selected scenario manifest promotes this boundary owner.,scenario initialization and boundary-version witness,"Scope does not transfer state, action, or capacity ownership.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.scoped_to.cross_border.chokepoint_shipping.hormuz,scoped_to,mechanism.cross_border.chokepoint_shipping,region.hormuz,scenario.middle_east_shipping_escalation,region.hormuz,scenario initialization witness,Active only while the selected scenario manifest promotes this boundary owner.,scenario initialization and boundary-version witness,"Scope does not transfer state, action, or capacity ownership.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.scoped_to.de.government.germany,scoped_to,institution.de.government,sovereign.germany,scenario.euro_sovereign_bank_stress,sovereign.germany,scenario initialization witness,Active only while the selected scenario manifest promotes this boundary owner.,scenario initialization and boundary-version witness,"Scope does not transfer state, action, or capacity ownership.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.scoped_to.eu.commission.euro_area,scoped_to,institution.eu.commission,federated.euro_area,scenario.euro_sovereign_bank_stress,federated.euro_area,scenario initialization witness,Active only while the selected scenario manifest promotes this boundary owner.,scenario initialization and boundary-version witness,"Scope does not transfer state, action, or capacity ownership.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.scoped_to.eu.ecb.euro_area,scoped_to,institution.eu.ecb,federated.euro_area,scenario.euro_sovereign_bank_stress,federated.euro_area,scenario initialization witness,Active only while the selected scenario manifest promotes this boundary owner.,scenario initialization and boundary-version witness,"Scope does not transfer state, action, or capacity ownership.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.scoped_to.eu.exposed_banks.euro_area,scoped_to,cohort.eu.exposed_banks,federated.euro_area,scenario.euro_sovereign_bank_stress,federated.euro_area,scenario initialization witness,Active only while the selected scenario manifest promotes this boundary owner.,scenario initialization and boundary-version witness,"Scope does not transfer state, action, or capacity ownership.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.scoped_to.eu.member_governments.euro_area,scoped_to,institution.eu.member_governments,federated.euro_area,scenario.euro_sovereign_bank_stress,federated.euro_area,scenario initialization witness,Active only while the selected scenario manifest promotes this boundary owner.,scenario initialization and boundary-version witness,"Scope does not transfer state, action, or capacity ownership.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.scoped_to.eu.national_central_banks.euro_area,scoped_to,institution.eu.national_central_banks,federated.euro_area,scenario.euro_sovereign_bank_stress,federated.euro_area,scenario initialization witness,Active only while the selected scenario manifest promotes this boundary owner.,scenario initialization and boundary-version witness,"Scope does not transfer state, action, or capacity ownership.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.scoped_to.fr.government.france,scoped_to,institution.fr.government,sovereign.france,scenario.euro_sovereign_bank_stress,sovereign.france,scenario initialization witness,Active only while the selected scenario manifest promotes this boundary owner.,scenario initialization and boundary-version witness,"Scope does not transfer state, action, or capacity ownership.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.scoped_to.il.bank_of_israel.israel,scoped_to,institution.il.bank_of_israel,sovereign.israel,scenario.middle_east_shipping_escalation,sovereign.israel,scenario initialization witness,Active only while the selected scenario manifest promotes this boundary owner.,scenario initialization and boundary-version witness,"Scope does not transfer state, action, or capacity ownership.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.scoped_to.il.cabinet.israel,scoped_to,body.il.cabinet,sovereign.israel,scenario.middle_east_shipping_escalation,sovereign.israel,scenario initialization witness,Active only while the selected scenario manifest promotes this boundary owner.,scenario initialization and boundary-version witness,"Scope does not transfer state, action, or capacity ownership.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.scoped_to.il.security.israel,scoped_to,institution.il.security,sovereign.israel,scenario.middle_east_shipping_escalation,sovereign.israel,scenario initialization witness,Active only while the selected scenario manifest promotes this boundary owner.,scenario initialization and boundary-version witness,"Scope does not transfer state, action, or capacity ownership.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.scoped_to.ir.central_bank.iran,scoped_to,institution.ir.central_bank,sovereign.iran,scenario.middle_east_shipping_escalation,sovereign.iran,scenario initialization witness,Active only while the selected scenario manifest promotes this boundary owner.,scenario initialization and boundary-version witness,"Scope does not transfer state, action, or capacity ownership.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.scoped_to.ir.irgc.iran,scoped_to,institution.ir.irgc,sovereign.iran,scenario.middle_east_shipping_escalation,sovereign.iran,scenario initialization witness,Active only while the selected scenario manifest promotes this boundary owner.,scenario initialization and boundary-version witness,"Scope does not transfer state, action, or capacity ownership.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.scoped_to.ir.oil.iran,scoped_to,institution.ir.oil,sovereign.iran,scenario.middle_east_shipping_escalation,sovereign.iran,scenario initialization witness,Active only while the selected scenario manifest promotes this boundary owner.,scenario initialization and boundary-version witness,"Scope does not transfer state, action, or capacity ownership.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.scoped_to.it.government.italy,scoped_to,institution.it.government,sovereign.italy,scenario.euro_sovereign_bank_stress,sovereign.italy,scenario initialization witness,Active only while the selected scenario manifest promotes this boundary owner.,scenario initialization and boundary-version witness,"Scope does not transfer state, action, or capacity ownership.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.scoped_to.jp.banks.japan,scoped_to,cohort.jp.banks,sovereign.japan,scenario.japan_duration_fx,sovereign.japan,scenario initialization witness,Active only while the selected scenario manifest promotes this boundary owner.,scenario initialization and boundary-version witness,"Scope does not transfer state, action, or capacity ownership.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.scoped_to.jp.boj.japan,scoped_to,institution.jp.boj,sovereign.japan,scenario.japan_duration_fx,sovereign.japan,scenario initialization witness,Active only while the selected scenario manifest promotes this boundary owner.,scenario initialization and boundary-version witness,"Scope does not transfer state, action, or capacity ownership.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.scoped_to.jp.insurers.japan,scoped_to,cohort.jp.insurers,sovereign.japan,scenario.japan_duration_fx,sovereign.japan,scenario initialization witness,Active only while the selected scenario manifest promotes this boundary owner.,scenario initialization and boundary-version witness,"Scope does not transfer state, action, or capacity ownership.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.scoped_to.jp.mof.japan,scoped_to,institution.jp.mof,sovereign.japan,scenario.japan_duration_fx,sovereign.japan,scenario initialization witness,Active only while the selected scenario manifest promotes this boundary owner.,scenario initialization and boundary-version witness,"Scope does not transfer state, action, or capacity ownership.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.scoped_to.jp.pensions.japan,scoped_to,cohort.jp.pensions,sovereign.japan,scenario.japan_duration_fx,sovereign.japan,scenario initialization witness,Active only while the selected scenario manifest promotes this boundary owner.,scenario initialization and boundary-version witness,"Scope does not transfer state, action, or capacity ownership.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.scoped_to.qa.central_bank.qatar,scoped_to,institution.qa.central_bank,sovereign.qatar,scenario.gulf_oil_dollar_peg,sovereign.qatar,scenario initialization witness,Active only while the selected scenario manifest promotes this boundary owner.,scenario initialization and boundary-version witness,"Scope does not transfer state, action, or capacity ownership.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.scoped_to.qa.energy_companies.qatar,scoped_to,institution.qa.energy_companies,sovereign.qatar,scenario.gulf_oil_dollar_peg,sovereign.qatar,scenario initialization witness,Active only while the selected scenario manifest promotes this boundary owner.,scenario initialization and boundary-version witness,"Scope does not transfer state, action, or capacity ownership.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.scoped_to.qa.sovereign_funds.qatar,scoped_to,institution.qa.sovereign_funds,sovereign.qatar,scenario.gulf_oil_dollar_peg,sovereign.qatar,scenario initialization witness,Active only while the selected scenario manifest promotes this boundary owner.,scenario initialization and boundary-version witness,"Scope does not transfer state, action, or capacity ownership.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.scoped_to.ru.central_bank.russia,scoped_to,institution.ru.central_bank,sovereign.russia,scenario.russia_energy_sanctions,sovereign.russia,scenario initialization witness,Active only while the selected scenario manifest promotes this boundary owner.,scenario initialization and boundary-version witness,"Scope does not transfer state, action, or capacity ownership.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.scoped_to.ru.energy_firms.russia,scoped_to,cohort.ru.energy_firms,sovereign.russia,scenario.russia_energy_sanctions,sovereign.russia,scenario initialization witness,Active only while the selected scenario manifest promotes this boundary owner.,scenario initialization and boundary-version witness,"Scope does not transfer state, action, or capacity ownership.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.scoped_to.ru.presidency.russia,scoped_to,institution.ru.presidency,sovereign.russia,scenario.russia_energy_sanctions,sovereign.russia,scenario initialization witness,Active only while the selected scenario manifest promotes this boundary owner.,scenario initialization and boundary-version witness,"Scope does not transfer state, action, or capacity ownership.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.scoped_to.ru.security.russia,scoped_to,institution.ru.security,sovereign.russia,scenario.russia_energy_sanctions,sovereign.russia,scenario initialization witness,Active only while the selected scenario manifest promotes this boundary owner.,scenario initialization and boundary-version witness,"Scope does not transfer state, action, or capacity ownership.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.scoped_to.sa.energy_ministry.saudi_arabia,scoped_to,institution.sa.energy_ministry,sovereign.saudi_arabia,scenario.gulf_oil_dollar_peg,sovereign.saudi_arabia,scenario initialization witness,Active only while the selected scenario manifest promotes this boundary owner.,scenario initialization and boundary-version witness,"Scope does not transfer state, action, or capacity ownership.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.scoped_to.sa.sama.saudi_arabia,scoped_to,institution.sa.sama,sovereign.saudi_arabia,scenario.gulf_oil_dollar_peg,sovereign.saudi_arabia,scenario initialization witness,Active only while the selected scenario manifest promotes this boundary owner.,scenario initialization and boundary-version witness,"Scope does not transfer state, action, or capacity ownership.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.scoped_to.sa.saudi_aramco.saudi_arabia,scoped_to,firm.sa.saudi_aramco,sovereign.saudi_arabia,scenario.gulf_oil_dollar_peg,sovereign.saudi_arabia,scenario initialization witness,Active only while the selected scenario manifest promotes this boundary owner.,scenario initialization and boundary-version witness,"Scope does not transfer state, action, or capacity ownership.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.scoped_to.sa.sovereign_funds.saudi_arabia,scoped_to,institution.sa.sovereign_funds,sovereign.saudi_arabia,scenario.gulf_oil_dollar_peg,sovereign.saudi_arabia,scenario initialization witness,Active only while the selected scenario manifest promotes this boundary owner.,scenario initialization and boundary-version witness,"Scope does not transfer state, action, or capacity ownership.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.scoped_to.sg.dollar_funding_banks.singapore,scoped_to,cohort.sg.dollar_funding_banks,sovereign.singapore,scenario.singapore_dollar_funding_shipping,sovereign.singapore,scenario initialization witness,Active only while the selected scenario manifest promotes this boundary owner.,scenario initialization and boundary-version witness,"Scope does not transfer state, action, or capacity ownership.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.scoped_to.sg.mas.singapore,scoped_to,institution.sg.mas,sovereign.singapore,scenario.singapore_dollar_funding_shipping,sovereign.singapore,scenario initialization witness,Active only while the selected scenario manifest promotes this boundary owner.,scenario initialization and boundary-version witness,"Scope does not transfer state, action, or capacity ownership.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.scoped_to.sg.shipping.singapore,scoped_to,institution.sg.shipping,sovereign.singapore,scenario.singapore_dollar_funding_shipping,sovereign.singapore,scenario initialization witness,Active only while the selected scenario manifest promotes this boundary owner.,scenario initialization and boundary-version witness,"Scope does not transfer state, action, or capacity ownership.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.scoped_to.sg.sovereign_funds.singapore,scoped_to,institution.sg.sovereign_funds,sovereign.singapore,scenario.singapore_dollar_funding_shipping,sovereign.singapore,scenario initialization witness,Active only while the selected scenario manifest promotes this boundary owner.,scenario initialization and boundary-version witness,"Scope does not transfer state, action, or capacity ownership.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.scoped_to.uk.boe.united_kingdom,scoped_to,institution.uk.boe,sovereign.united_kingdom,scenario.uk_gilt_ldi_stress,sovereign.united_kingdom,scenario initialization witness,Active only while the selected scenario manifest promotes this boundary owner.,scenario initialization and boundary-version witness,"Scope does not transfer state, action, or capacity ownership.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.scoped_to.uk.dmo.united_kingdom,scoped_to,institution.uk.dmo,sovereign.united_kingdom,scenario.uk_gilt_ldi_stress,sovereign.united_kingdom,scenario initialization witness,Active only while the selected scenario manifest promotes this boundary owner.,scenario initialization and boundary-version witness,"Scope does not transfer state, action, or capacity ownership.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.scoped_to.uk.government.united_kingdom,scoped_to,institution.uk.government,sovereign.united_kingdom,scenario.uk_gilt_ldi_stress,sovereign.united_kingdom,scenario initialization witness,Active only while the selected scenario manifest promotes this boundary owner.,scenario initialization and boundary-version witness,"Scope does not transfer state, action, or capacity ownership.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
rel.scoped_to.uk.pensions.united_kingdom,scoped_to,cohort.uk.pensions,sovereign.united_kingdom,scenario.uk_gilt_ldi_stress,sovereign.united_kingdom,scenario initialization witness,Active only while the selected scenario manifest promotes this boundary owner.,scenario initialization and boundary-version witness,"Scope does not transfer state, action, or capacity ownership.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/closure/research_backlog.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/closure/research_backlog.csv
size_bytes: 186155
mode_octal: "0644"
modified_at_utc: 2026-09-04T04:11:43.346866Z
sha256: cef339a3b25243d5eed41cc027f35b43de4de352be0c9ad9e1cfb8ff4502657a
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
backlog_id,catalog_id,artifact_kind,artifact_key,blocker_code,required_evidence,provenance
backlog.catalog.000741a82339,mechanism.flow.global.pork,owned_state,state.pork_flow.losses,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:615
backlog.catalog.006c535219aa,inst.us.bank.burrow,transmission,tx.burrow.lending_posture.to_borrowers,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",06-design-discussion-representation-catalog.md:881-884;07-design-discussion-burrow-composition-probe.md:291
backlog.catalog.00c93c67f9f5,institution.jp.mof,owned_state,state.jp.mof.currency,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.02f4f6a3b35f,UNKNOWN,owned_state,state.external_region.southeast_asia.dollar_funding,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.031f56f3f6e5,mechanism.cross_border.chokepoint_shipping,owned_state,state.cross_border.chokepoint_shipping.commodities,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.03307e477579,UNKNOWN,transmission,tx.economic_boundary.external_region.southeast_asia.trade.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.03c23b7195c2,mechanism.transform.global.coffee_roasting,owned_state,state.coffee_roasting.capacity,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:611
backlog.catalog.0400d5fb9d01,mechanism.transform.global.coffee_roasting,owned_state,queue.coffee_roasting.work,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:611
backlog.catalog.043e2843f863,UNKNOWN,transmission,tx.economic_boundary.external_region.gulf.commodities.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.0471cc3e1749,UNKNOWN,transmission,tx.economic_boundary.external_region.eastern_europe.reserves.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.04ede88337aa,network.media.honkbox,transmission,tx.media.honkbox.gnbc,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",task.md:818-826;05-design-discussion-representation-bible.md:184-185
backlog.catalog.056ed200d9c5,cohort.eu.exposed_banks,owned_state,state.eu.exposed_banks.financial_stress,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.05a75a91e5a2,reference.us.nyfed.sofr,owned_state,state.sofr.publication,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:1030-1043
backlog.catalog.066d8b71f7c2,schedule.us.federal_reserve.fomc,owned_state,state.fomc.calendar,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:377;07-design-discussion-burrow-composition-probe.md:171
backlog.catalog.074d660da9e4,institution.sg.sovereign_funds,owned_state,state.sg.sovereign_funds.reserves,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.07b760350578,mechanism.cross_border.chokepoint_shipping,transmission,tx.economic_boundary.cross_border.chokepoint_shipping.trade.external_region.middle_east,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.07bc12b30a70,institution.us.member.unions,relationship,rel.represents.member.unions.workers.by.sector,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",06-design-discussion-representation-catalog.md:309
backlog.catalog.080d1392799b,inst.us.treasury,owned_state,state.treasury.cash_balance,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:215;04-design-discussion-minimum-simulation-kernel.md:717
backlog.catalog.097f31cb22f8,UNKNOWN,transmission,tx.published_references.to_burrow,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",06-design-discussion-representation-catalog.md:877-884;06-design-discussion-representation-catalog.md:904;07-design-discussion-burrow-composition-probe.md:182
backlog.catalog.09b26a6e62f2,cohort.jp.banks,transmission,tx.economic_boundary.jp.banks.financial_stress.external_region.east_asia,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.09e1a833fd12,UNKNOWN,owned_state,state.external_region.middle_east.trade,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.0a07174440ad,institution.ir.central_bank,owned_state,state.ir.central_bank.policy_stance,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.0a54e8da3fe4,body.il.cabinet,owned_state,state.il.cabinet.trade,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.0b0a8d0f8f07,UNKNOWN,owned_state,state.external_region.southeast_asia.reserves,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.0b1d3eb42bec,cohort.sg.dollar_funding_banks,owned_state,state.sg.dollar_funding_banks.dollar_funding,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.0b1e5cddcdbc,mechanism.cross_border.chokepoint_shipping,owned_state,state.cross_border.chokepoint_shipping.trade,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.0c3286a55385,institution.ae.central_bank,owned_state,state.ae.central_bank.currency,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.0c61261650fb,institution.eu.ecb,transmission,tx.economic_boundary.eu.ecb.financial_stress.external_region.aggregated_europe,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.0cd512492a06,institution.ru.central_bank,owned_state,state.ru.central_bank.currency,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.0d1f58a98811,institution.sa.sama,transmission,tx.economic_boundary.sa.sama.currency.external_region.gulf,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.0d938c2d3715,cohort.sg.dollar_funding_banks,transmission,tx.economic_boundary.sg.dollar_funding_banks.financial_stress.external_region.southeast_asia,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.0e0087bd38cd,institution.us.homebuilders,relationship,rel.represents.homebuilders.homebuilders,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",06-design-discussion-representation-catalog.md:306
backlog.catalog.0eff63952950,UNKNOWN,transmission,tx.economic_boundary.external_region.aggregated_europe.policy_stance.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.0f8337cc9175,institution.jp.boj,owned_state,state.jp.boj.currency,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.1056ae9d4347,inst.us.bank.burrow,relationship,rel.burrow.supervised_by_fed,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",06-design-discussion-representation-catalog.md:872-876;06-design-discussion-representation-catalog.md:902
backlog.catalog.11306e291ba8,institution.jp.boj,transmission,tx.economic_boundary.jp.boj.policy_stance.external_region.east_asia,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.115d4c4be5a5,institution.ae.central_bank,owned_state,state.ae.central_bank.reserves,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.122c42f4ec99,poplens.media.honkbox.posting_population,relationship,rel.projects.media.honkbox_posting_population.honkbox,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",06-design-discussion-representation-catalog.md:373;06-design-discussion-representation-catalog.md:982-985
backlog.catalog.1268ae0de756,market.product.global.timber,owned_state,state.lumber_market.clearing,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:617
backlog.catalog.12dae18b59d0,institution.us.consumer.finance.advocates,relationship,rel.represents.consumer.finance.advocates.low.income.underserved.borrowers,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",06-design-discussion-representation-catalog.md:310
backlog.catalog.132651154103,UNKNOWN,transmission,tx.economic_boundary.external_region.central_africa.policy_stance.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.15edfe61a820,cohort.jp.banks,owned_state,state.jp.banks.dollar_funding,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.17a3262d23eb,UNKNOWN,owned_state,state.external_region.gulf.commodities,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.17e887562c76,industry.amazon_basin.pork,owned_state,state.amazon_hogs.water_access,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:609
backlog.catalog.18b5dc3428a6,industry.amazon_basin.coffee,owned_state,account.amazon_coffee.finance,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:607
backlog.catalog.18f88170818a,institution.sg.sovereign_funds,transmission,tx.economic_boundary.sg.sovereign_funds.reserves.external_region.southeast_asia,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.18fc9b9f1d78,inst.us.treasury,relationship,rel.schedule.treasury_auction,lifecycle,Source-backed effective period and exit conditions.,06-design-discussion-representation-catalog.md:377
backlog.catalog.193b93649f00,facility.us.federal_reserve.srf,owned_state,state.srf.readiness,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:1013-1015
backlog.catalog.19435cf5a52d,UNKNOWN,transmission,tx.economic_boundary.external_region.east_asia.currency.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.1c40f3f1d7cb,UNKNOWN,transmission,tx.economic_boundary.external_region.southeast_asia.reserves.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.1c8262ea6ebf,mechanism.measurement.us.consumer_prices,owned_state,queue.us_prices.releases,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:621
backlog.catalog.1dd5fe913541,institution.us.chamber.of.commerce,relationship,rel.represents.chamber.commerce.business.generally,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",06-design-discussion-representation-catalog.md:308
backlog.catalog.1e073cb3112b,UNKNOWN,owned_state,state.external_region.eastern_europe.trade,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.1efe916cf229,institution.media.honkbox,relationship,rel.operates.media.honkbox_platform.honkbox_network,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",06-design-discussion-representation-catalog.md:373;06-design-discussion-representation-catalog.md:982-985
backlog.catalog.1f1a7726d221,UNKNOWN,transmission,tx.economic_boundary.external_region.aggregated_europe.financial_stress.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.1f8198106ee3,UNKNOWN,transmission,tx.economic_boundary.external_region.middle_east.policy_stance.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.204e1a598f9c,UNKNOWN,transmission,tx.economic_boundary.external_region.gulf.policy_stance.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.20a27ef8131c,institution.fr.government,owned_state,state.fr.government.policy_stance,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.20adcf5cd08c,institution.ae.central_bank,action_domain,currency,owner_or_content,Resolved owner and source-backed action contract.,04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.21006aa3a539,UNKNOWN,transmission,tx.economic_boundary.external_region.eastern_europe.dollar_funding.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.217203f75287,UNKNOWN,transmission,tx.economic_boundary.external_region.middle_east.trade.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.21b3ee1d066a,adapter.real.us.food_baskets,transmission,tx.us_food_baskets.to_price_measurement,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",06-design-discussion-representation-catalog.md:682
backlog.catalog.226487f725e3,adapter.real.us.construction_inputs,transmission,tx.us_construction.to_housing_activity,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",06-design-discussion-representation-catalog.md:681
backlog.catalog.2265fc5c0195,institution.ir.irgc,transmission,tx.economic_boundary.ir.irgc.commodities.external_region.middle_east,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.227689fa1950,institution.ir.irgc,owned_state,state.ir.irgc.trade,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.22b063ed9cdc,market.product.global.pork,owned_state,state.pork_market.orders,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:617
backlog.catalog.22e09e55c568,UNKNOWN,transmission,tx.economic_boundary.external_region.east_asia.trade.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.23c2dec30d8b,staff.us.federal_reserve.ny_markets_group,relationship,rel.operation.ny_markets_soma,lifecycle,Source-backed effective period and exit conditions.,06-design-discussion-representation-catalog.md:261;07-design-discussion-burrow-composition-probe.md:144
backlog.catalog.2578959f6b83,process.climate_agriculture.amazon_basin,owned_state,state.amazon_process.access,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:605
backlog.catalog.26009f2487c9,UNKNOWN,owned_state,state.external_region.southeast_asia.currency,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.26a1bbe1e025,cohort.eu.exposed_banks,action_domain,dollar_funding,owner_or_content,Resolved owner and source-backed action contract.,04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.276dc905d5c3,legal.us.federal_reserve.section_13_3,owned_state,state.13_3.period_variant,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:1519-1521;07-design-discussion-burrow-composition-probe.md:160-173
backlog.catalog.2950e4eab6c2,UNKNOWN,relationship,rel.aggregates_into.external_region.middle_east.global,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.295bb980a464,UNKNOWN,owned_state,state.external_region.central_africa.dollar_funding,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.29b860c159e7,institution.ir.central_bank,owned_state,state.ir.central_bank.currency,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.29c1e46d8a37,institution.qa.central_bank,transmission,tx.economic_boundary.qa.central_bank.currency.external_region.gulf,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.2a9599ad0998,schedule.us.federal_reserve.blackout,owned_state,state.fomc.blackout,state_transition,"Source-backed transition, witness, owner, and conserved unit.",07-design-discussion-burrow-composition-probe.md:165-173
backlog.catalog.2b5d0232dc03,institution.sg.sovereign_funds,owned_state,state.sg.sovereign_funds.dollar_funding,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.2bdaef39b1d1,institution.cn.pboc,owned_state,state.cn.pboc.financial_stress,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.2c389c092eab,institution.il.bank_of_israel,transmission,tx.economic_boundary.il.bank_of_israel.currency.external_region.middle_east,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.2c589b2d1750,mechanism.transform.global.coffee_roasting,owned_state,state.coffee_roasting.losses,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:611
backlog.catalog.2c8aeefcd22b,stateful.us.treasury.duration_supply,transmission,tx.supply.duration.treasury_secondary,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:173-184
backlog.catalog.2c981cd0b38e,industry.amazon_basin.pork,owned_state,state.amazon_hogs.health,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:609
backlog.catalog.2caaf6d30e7a,schedule.us.federal_reserve.fomc,relationship,rel.derivation.fomc_blackout,lifecycle,Source-backed effective period and exit conditions.,07-design-discussion-burrow-composition-probe.md:165-173
backlog.catalog.2e3f6f7aa77e,adapter.real.us.construction_inputs,owned_state,state.us_construction.lumber_allocation,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:618
backlog.catalog.2e45d2b89d45,UNKNOWN,transmission,tx.economic_boundary.external_region.central_africa.dollar_funding.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.2e59a5ec7460,adapter.real.us.construction_inputs,owned_state,account.us_construction.external,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:618
backlog.catalog.2e6771ba34c9,inst.us.federal_reserve.reserve_banks_other,relationship,rel.membership.federal_reserve.other_banks_system,lifecycle,Source-backed effective period and exit conditions.,06-design-discussion-representation-catalog.md:269;06-design-discussion-representation-catalog.md:960
backlog.catalog.2ec411408cce,industry.amazon_basin.pork,owned_state,state.amazon_hogs.capacity,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:609
backlog.catalog.2efc38108568,market.us.treasury.auction,transmission,tx.auction.treasury.market_evidence,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",task.md:49;task.md:72;task.md:101
backlog.catalog.2f093bd73064,UNKNOWN,owned_state,state.external_region.eastern_europe.reserves,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.2f17e7bcd26b,mechanism.transform.global.hogs_slaughter,owned_state,state.hogs_slaughter.inspection,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:612
backlog.catalog.2f3aada1577c,reference.us.nyfed.sofr,transmission,tx.sofr.repo.contracts,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",06-design-discussion-representation-catalog.md:1030-1032;07-design-discussion-burrow-composition-probe.md:182
backlog.catalog.30319ecca09e,institution.ae.sovereign_funds,owned_state,state.ae.sovereign_funds.reserves,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.3075c6d93e7b,institution.ru.central_bank,transmission,tx.economic_boundary.ru.central_bank.currency.external_region.eastern_europe,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.30c291ca7b8f,institution.us.bank.policy.institute,relationship,rel.represents.bank.policy.institute.large.banks,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",06-design-discussion-representation-catalog.md:303
backlog.catalog.3187fca785f4,institution.ir.central_bank,transmission,tx.economic_boundary.ir.central_bank.reserves.external_region.middle_east,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.31e8d419b49d,mechanism.measurement.us.consumer_prices,owned_state,state.us_prices.weights,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:621
backlog.catalog.32188adbeacd,UNKNOWN,owned_state,state.external_region.gulf.trade,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.327a68531c11,UNKNOWN,transmission,tx.economic_boundary.external_region.southeast_asia.currency.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.3319a4cc9039,UNKNOWN,owned_state,state.external_region.latin_america.commodities,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.33d7c1c0e527,UNKNOWN,transmission,tx.economic_boundary.external_region.eastern_europe.currency.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.3485cc9d4eaf,institution.sg.mas,transmission,tx.economic_boundary.sg.mas.dollar_funding.external_region.southeast_asia,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.34b56acd73d3,institution.cn.policy_banks,owned_state,state.cn.policy_banks.dollar_funding,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.35c2f79da562,UNKNOWN,owned_state,state.external_region.aggregated_europe.dollar_funding,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.35ef315207b8,market.product.global.pork,owned_state,state.pork_market.residual,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:617
backlog.catalog.35fc977dcffb,UNKNOWN,transmission,tx.economic_boundary.external_region.latin_america.policy_stance.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.36602eca9de5,institution.uk.boe,transmission,tx.economic_boundary.uk.boe.dollar_funding.external_region.aggregated_europe,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.36a9c1b1343a,process.climate_agriculture.amazon_basin,owned_state,state.amazon_process.hydrothermal,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:605
backlog.catalog.36e6655df1b2,process.climate_agriculture.amazon_basin,transmission,tx.amazon_process.to_forestry,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",06-design-discussion-representation-catalog.md:647
backlog.catalog.371989da8c03,UNKNOWN,owned_state,state.external_region.aggregated_europe.growth_demand,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.374e26bcd658,institution.us.bea,relationship,rel.publishes.us.bea.pce,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",06-design-discussion-representation-catalog.md:1043;05-design-discussion-representation-bible.md:183;05-design-discussion-representation-bible.md:317
backlog.catalog.3776a078ad92,process.climate_agriculture.amazon_basin,transmission,tx.amazon_process.to_feed_boundary,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",06-design-discussion-representation-catalog.md:650
backlog.catalog.37c288dce9c7,institution.uk.dmo,transmission,tx.economic_boundary.uk.dmo.financial_stress.external_region.aggregated_europe,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.37f3603e105f,institution.ru.presidency,transmission,tx.economic_boundary.ru.presidency.policy_stance.external_region.eastern_europe,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.381ff6bd379d,UNKNOWN,transmission,tx.economic_boundary.external_region.latin_america.growth_demand.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.3825caad2216,UNKNOWN,owned_state,state.external_region.middle_east.currency,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.38a7e9c95b04,UNKNOWN,owned_state,state.external_region.eastern_europe.policy_stance,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.38eddf5f81eb,UNKNOWN,owned_state,state.external_region.latin_america.policy_stance,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.39991f373180,market.us.treasury.secondary,transmission,tx.treasury.collateral.repo,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:178-180
backlog.catalog.3a761ed8e151,UNKNOWN,relationship,rel.aggregates_into.external_region.gulf.global,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.3b284bc4efbe,UNKNOWN,owned_state,state.external_region.aggregated_europe.trade,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.3b80f83ce7c0,agreement.us.federal_reserve.swap_lines,relationship,rel.agreement.swap_lines_federal_reserve,lifecycle,Source-backed effective period and exit conditions.,06-design-discussion-representation-catalog.md:339
backlog.catalog.3b822db4d5a9,institution.il.security,owned_state,state.il.security.commodities,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.3c532c86cc85,institution.sa.sama,transmission,tx.economic_boundary.sa.sama.reserves.external_region.gulf,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.3d4fda082d18,inst.us.cme,relationship,rel.operation.cme_treasury_futures,lifecycle,Source-backed effective period and exit conditions.,06-design-discussion-representation-catalog.md:345
backlog.catalog.3ef391afc1fa,mechanism.flow.global.timber,owned_state,queue.lumber_flow.delivery,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:613
backlog.catalog.3f149f8e6dc8,institution.ae.central_bank,transmission,tx.economic_boundary.ae.central_bank.reserves.external_region.gulf,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.3f7976f96a73,institution.il.security,transmission,tx.economic_boundary.il.security.trade.external_region.middle_east,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.3f7a684da371,institution.cn.pboc,owned_state,state.cn.pboc.dollar_funding,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.3fe84402e9e0,UNKNOWN,transmission,tx.economic_boundary.external_region.middle_east.growth_demand.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.4000901e1260,UNKNOWN,transmission,tx.economic_boundary.external_region.middle_east.currency.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.402b69744026,UNKNOWN,transmission,tx.economic_boundary.external_region.latin_america.dollar_funding.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.4155f753cb7c,UNKNOWN,owned_state,state.external_region.southeast_asia.trade,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.4217a55ec869,cohort.ru.energy_firms,owned_state,state.ru.energy_firms.commodities,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.423aea8a2dd2,cohort.jp.insurers,owned_state,state.jp.insurers.reserves,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.426026f96565,institution.eu.national_central_banks,transmission,tx.economic_boundary.eu.national_central_banks.financial_stress.external_region.aggregated_europe,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.4272444b84c3,inst.us.federal_reserve.board,relationship,rel.membership.federal_reserve.board_system,lifecycle,Source-backed effective period and exit conditions.,06-design-discussion-representation-catalog.md:269;06-design-discussion-representation-catalog.md:960
backlog.catalog.42c5d3616de7,institution.qa.energy_companies,owned_state,state.qa.energy_companies.trade,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.433f9ef33f28,UNKNOWN,transmission,tx.economic_boundary.external_region.gulf.financial_stress.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.43cc0448e7c6,UNKNOWN,transmission,tx.economic_boundary.external_region.eastern_europe.trade.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.44299ea45970,market.product.global.coffee,owned_state,state.green_coffee_market.residual,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:617
backlog.catalog.4499638aca5d,adapter.real.us.food_baskets,entity_scope,adapter.real.us.food_baskets->UNKNOWN,scope,Resolved typed scope endpoint.,06-design-discussion-representation-catalog.md:521
backlog.catalog.44be98768f53,institution.media.aftv,relationship,rel.operates.media.aftv_company.aftv,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",06-design-discussion-representation-catalog.md:371;05-design-discussion-representation-bible.md:1691
backlog.catalog.44d7fba452d9,UNKNOWN,transmission,tx.economic_boundary.external_region.middle_east.dollar_funding.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.45b7ca675e41,UNKNOWN,owned_state,state.external_region.southeast_asia.growth_demand,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.46e43bf7ac6f,institution.it.government,transmission,tx.economic_boundary.it.government.policy_stance.external_region.aggregated_europe,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.46e78368cde0,UNKNOWN,owned_state,state.external_region.gulf.dollar_funding,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.47b8d3f878fc,institution.ir.oil,transmission,tx.economic_boundary.ir.oil.trade.external_region.middle_east,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.48104bf1dea1,UNKNOWN,owned_state,state.external_region.latin_america.reserves,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.48dad02159c7,institution.ae.energy_companies,transmission,tx.economic_boundary.ae.energy_companies.trade.external_region.gulf,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.4b11f02efa2d,UNKNOWN,owned_state,state.external_region.latin_america.growth_demand,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.4b29003d1f98,institution.jp.mof,transmission,tx.economic_boundary.jp.mof.reserves.external_region.east_asia,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.4b48970e647a,mechanism.flow.global.timber,owned_state,state.lumber_flow.losses,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:613
backlog.catalog.4c04c34b6d90,UNKNOWN,owned_state,state.external_region.southeast_asia.commodities,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.4c533c8f5bc6,UNKNOWN,owned_state,state.external_region.central_africa.growth_demand,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.4c8e7d8e43be,institution.ae.energy_companies,owned_state,state.ae.energy_companies.commodities,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.4c9fd71cbac5,cohort.jp.insurers,action_domain,reserves,owner_or_content,Resolved owner and source-backed action contract.,04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.4d228dd7eedf,body.il.cabinet,transmission,tx.economic_boundary.il.cabinet.trade.external_region.middle_east,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.4de2748b73cf,body.il.cabinet,action_domain,trade,owner_or_content,Resolved owner and source-backed action contract.,04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.4ed431f3adf4,industry.amazon_basin.pork,owned_state,account.amazon_hogs.finance,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:609
backlog.catalog.504dba3490b9,institution.ir.irgc,owned_state,state.ir.irgc.commodities,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.50655e621c19,institution.sa.energy_ministry,transmission,tx.economic_boundary.sa.energy_ministry.policy_stance.external_region.gulf,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.519ec7da17b0,institution.ru.central_bank,transmission,tx.economic_boundary.ru.central_bank.reserves.external_region.eastern_europe,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.51f673ae48d7,institution.sg.mas,owned_state,state.sg.mas.dollar_funding,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.521c7e551997,UNKNOWN,transmission,tx.economic_boundary.external_region.aggregated_europe.trade.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.5222b32823d5,institution.ir.central_bank,transmission,tx.economic_boundary.ir.central_bank.policy_stance.external_region.middle_east,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.523ae86af76d,industry.amazon_basin.forestry,owned_state,state.amazon_forestry.capacity,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:606
backlog.catalog.524776408c32,institution.us.mortgage.bankers,relationship,rel.represents.mortgage.bankers.mortgage.originators,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",06-design-discussion-representation-catalog.md:306
backlog.catalog.52f315bcb300,UNKNOWN,owned_state,state.external_region.gulf.policy_stance,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.5310b509bf46,institution.us.american.bankers.association,relationship,rel.represents.american.bankers.association.all.banks,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",06-design-discussion-representation-catalog.md:303
backlog.catalog.535646aa7c0a,body.us.treasury.tbac,relationship,rel.advisory.tbac_treasury,lifecycle,Source-backed effective period and exit conditions.,06-design-discussion-representation-catalog.md:259
backlog.catalog.54ba9ada0a66,institution.de.government,transmission,tx.economic_boundary.de.government.financial_stress.external_region.aggregated_europe,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.56316bcd8e17,institution.cn.pboc,transmission,tx.economic_boundary.cn.pboc.currency.external_region.east_asia,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.56ada5030cc7,institution.de.government,transmission,tx.economic_boundary.de.government.policy_stance.external_region.aggregated_europe,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.56b3f16f0bf3,cohort.uk.pensions,owned_state,state.uk.pensions.dollar_funding,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.57313fd213d7,market.product.global.pork,owned_state,state.pork_market.failure,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:617
backlog.catalog.57813c791539,UNKNOWN,transmission,tx.market_observations.to_burrow,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",06-design-discussion-representation-catalog.md:877-884;06-design-discussion-representation-catalog.md:904;07-design-discussion-burrow-composition-probe.md:122;07-design-discussion-burrow-composition-probe.md:181
backlog.catalog.57c2995fd501,industry.amazon_basin.coffee,owned_state,state.amazon_coffee.capacity,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:607
backlog.catalog.57d01fe67b0d,adapter.real.us.food_baskets,owned_state,state.us_food.pork_allocation,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:619
backlog.catalog.57dec84cbba4,institution.uk.boe,transmission,tx.economic_boundary.uk.boe.policy_stance.external_region.aggregated_europe,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.5992dc4b8c8e,cohort.eu.exposed_banks,transmission,tx.economic_boundary.eu.exposed_banks.dollar_funding.external_region.aggregated_europe,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.59fb3fce3a28,UNKNOWN,transmission,tx.economic_boundary.external_region.east_asia.commodities.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.5a418e6998f7,institution.ru.central_bank,owned_state,state.ru.central_bank.reserves,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.5ca8b545f247,institution.qa.energy_companies,transmission,tx.economic_boundary.qa.energy_companies.commodities.external_region.gulf,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.5d6284abeb7a,organization_cohort.market.credit_rating_agencies,relationship,rel.publishes.market.credit_rating_agencies.credit_ratings,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",06-design-discussion-representation-catalog.md:355;05-design-discussion-representation-bible.md:183
backlog.catalog.5d78d7297a18,institution.cn.state_council,owned_state,state.cn.state_council.policy_stance,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.5d8c267b868a,market.product.global.coffee,owned_state,state.green_coffee_market.orders,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:617
backlog.catalog.5e08570ca0b7,cohort.jp.pensions,owned_state,state.jp.pensions.reserves,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.5e791031fb61,UNKNOWN,owned_state,state.external_region.central_africa.policy_stance,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.5ef2b7fb5a73,UNKNOWN,transmission,tx.economic_boundary.external_region.latin_america.trade.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.5fa3a0416686,institution.ru.security,transmission,tx.economic_boundary.ru.security.trade.external_region.eastern_europe,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.6043046e4bb8,institution.media.wool_street_journal,relationship,rel.operates.media.wool_street_journal_company.wool_street_journal,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",06-design-discussion-representation-catalog.md:371;05-design-discussion-representation-bible.md:1691
backlog.catalog.604e402497fd,UNKNOWN,relationship,rel.aggregates_into.external_region.aggregated_europe.global,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.6121ff192118,cohort.jp.insurers,transmission,tx.economic_boundary.jp.insurers.financial_stress.external_region.east_asia,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.613cea8f907e,cohort.ru.energy_firms,owned_state,state.ru.energy_firms.trade,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.61d0f7e2e2ea,institution.eu.member_governments,owned_state,state.eu.member_governments.policy_stance,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.625a8be41e47,cohort.jp.insurers,owned_state,state.jp.insurers.financial_stress,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.63a45c9c2372,institution.qa.energy_companies,transmission,tx.economic_boundary.qa.energy_companies.trade.external_region.gulf,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.63d1855a0ef3,UNKNOWN,owned_state,state.external_region.aggregated_europe.policy_stance,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.642bbe1a4c01,inst.us.bank.burrow,transmission,tx.burrow.requests.to_funding,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",06-design-discussion-representation-catalog.md:881-884;07-design-discussion-burrow-composition-probe.md:139
backlog.catalog.64a50efabafa,record.us.treasury.indemnification_13_3,relationship,rel.indemnity.treasury_esf_13_3,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",06-design-discussion-representation-catalog.md:363
backlog.catalog.6539967c6c25,UNKNOWN,owned_state,state.external_region.east_asia.currency,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.65f178c16c08,institution.sa.sovereign_funds,owned_state,state.sa.sovereign_funds.dollar_funding,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.66432e3e65b6,cohort.jp.pensions,transmission,tx.economic_boundary.jp.pensions.financial_stress.external_region.east_asia,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.6721816bf520,adapter.real.us.construction_inputs,entity_scope,adapter.real.us.construction_inputs->UNKNOWN,scope,Resolved typed scope endpoint.,06-design-discussion-representation-catalog.md:520
backlog.catalog.67e29730c571,mechanism.measurement.us.consumer_prices,owned_state,state.us_prices.method,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:621
backlog.catalog.67f17c2d0c14,UNKNOWN,transmission,tx.settlement_results.to_burrow,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",06-design-discussion-representation-catalog.md:877-884;06-design-discussion-representation-catalog.md:904;07-design-discussion-burrow-composition-probe.md:290
backlog.catalog.68323aa96265,facility.us.federal_reserve.srf,transmission,tx.srf.counterparty_option,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",06-design-discussion-representation-catalog.md:1005-1016
backlog.catalog.684bdebe99ad,UNKNOWN,transmission,tx.economic_boundary.external_region.middle_east.commodities.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.6870641b6d5b,UNKNOWN,owned_state,state.external_region.middle_east.reserves,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.68a5fec426fe,cohort.us.bank.regional_cre_concentrated,entity_scope,cohort.us.bank.regional_cre_concentrated->UNKNOWN,scope,Resolved typed scope endpoint.,06-design-discussion-representation-catalog.md:914; intended subject: region.us
backlog.catalog.68fb132cca1e,office.media.aftv.anchor_chair,relationship,rel.belongs_to.media.aftv_anchor_chair.aftv,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",06-design-discussion-representation-catalog.md:1551;06-design-discussion-representation-catalog.md:1689
backlog.catalog.6942e6c31fc4,agreement.us.repo.bilateral,transmission,tx.repo.sofr.publication,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",06-design-discussion-representation-catalog.md:1030
backlog.catalog.69e08867aeb5,body.us.federal_reserve.board_voting,relationship,rel.governance.board_voting_board,lifecycle,Source-backed effective period and exit conditions.,06-design-discussion-representation-catalog.md:256
backlog.catalog.6a3001ecc967,mechanism.flow.global.coffee,owned_state,state.green_coffee_flow.losses,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:614
backlog.catalog.6a4a20316a97,institution.cn.state_council,transmission,tx.economic_boundary.cn.state_council.trade.external_region.east_asia,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.6b0d651ceb23,institution.ir.irgc,transmission,tx.economic_boundary.ir.irgc.trade.external_region.middle_east,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.6b9429368f7d,schedule.us.federal_reserve.blackout,transmission,tx.fomc.blackout.chair,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",07-design-discussion-burrow-composition-probe.md:165-173
backlog.catalog.6bd0e110176b,institution.de.government,owned_state,state.de.government.policy_stance,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.6bffd177f942,cohort.ru.energy_firms,transmission,tx.economic_boundary.ru.energy_firms.commodities.external_region.eastern_europe,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.6c42bed8d6c6,UNKNOWN,owned_state,state.external_region.east_asia.trade,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.6d4faec3f475,institution.us.independent.community.bankers,relationship,rel.represents.independent.community.bankers.community.banks.credit.unions,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",06-design-discussion-representation-catalog.md:303;06-design-discussion-representation-catalog.md:285
backlog.catalog.6d6c09ecccbc,industry.amazon_basin.forestry,owned_state,inventory.amazon_forestry.timber,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:606
backlog.catalog.6d76ba3dfbf3,institution.cn.safe,owned_state,state.cn.safe.reserves,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.6de98fadb21c,institution.jp.boj,transmission,tx.economic_boundary.jp.boj.currency.external_region.east_asia,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.6de9f6e6305e,institution.cn.party_leadership,owned_state,state.cn.party_leadership.policy_stance,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.6e4cbf52a8e1,UNKNOWN,relationship,rel.aggregates_into.external_region.southeast_asia.global,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.6fdbe39e0ec3,institution.ru.presidency,owned_state,state.ru.presidency.policy_stance,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.70f3612d773c,UNKNOWN,transmission,tx.economic_boundary.external_region.aggregated_europe.commodities.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.7118938f90f8,inst.us.treasury,owned_state,state.treasury.outstanding_debt,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:215;04-design-discussion-minimum-simulation-kernel.md:717
backlog.catalog.714cc9e2e546,institution.sg.mas,transmission,tx.economic_boundary.sg.mas.policy_stance.external_region.southeast_asia,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.7200d900ba68,federation.us.federal_reserve,owned_state,state.federal_reserve.consolidated_projection,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:269;06-design-discussion-representation-catalog.md:960
backlog.catalog.73d8cf8b44dc,institution.it.government,owned_state,state.it.government.policy_stance,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.75330a3d714f,UNKNOWN,owned_state,state.external_region.gulf.reserves,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.75e2a5b940f5,network.media.honkbox,transmission,tx.media.honkbox.aftv,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",task.md:818-826;05-design-discussion-representation-bible.md:184-185
backlog.catalog.75f1600d9a9e,institution.uk.boe,owned_state,state.uk.boe.financial_stress,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.76079a4ffb71,cohort.eu.exposed_banks,action_domain,financial_stress,owner_or_content,Resolved owner and source-backed action contract.,04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.769f57e0aa9b,cohort.uk.pensions,transmission,tx.economic_boundary.uk.pensions.dollar_funding.external_region.aggregated_europe,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.7723f077fdb0,institution.fr.government,transmission,tx.economic_boundary.fr.government.policy_stance.external_region.aggregated_europe,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.7748dff04a0f,agreement.us.repo.bilateral,owned_state,state.repo.exposure_index,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:335
backlog.catalog.77b198efc413,institution.cn.military,transmission,tx.economic_boundary.cn.military.trade.external_region.east_asia,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.785bd4520c1d,UNKNOWN,transmission,tx.economic_boundary.external_region.middle_east.reserves.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.786e6ce75a40,institution.cn.pboc,transmission,tx.economic_boundary.cn.pboc.financial_stress.external_region.east_asia,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.78af1ce73a0b,person.media.aftv_host,relationship,rel.holds.media.aftv_host.aftv_anchor_chair,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",06-design-discussion-representation-catalog.md:252;06-design-discussion-representation-catalog.md:1551-1558
backlog.catalog.797c3acbd17a,UNKNOWN,transmission,tx.economic_boundary.external_region.gulf.growth_demand.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.79b057b143b2,body.il.cabinet,transmission,tx.economic_boundary.il.cabinet.policy_stance.external_region.middle_east,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.79da0adb4ba3,industry.amazon_basin.forestry,relationship,rel.amazon_forestry.operates_in_basin,lifecycle,Source-backed effective period and exit conditions.,06-design-discussion-representation-catalog.md:734
backlog.catalog.7b63fe1a29c4,mechanism.transform.global.hogs_slaughter,owned_state,state.hogs_slaughter.losses,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:612
backlog.catalog.7bc868b1770f,UNKNOWN,relationship,rel.aggregates_into.external_region.eastern_europe.global,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.7bcfed5de093,industry.amazon_basin.coffee,relationship,rel.amazon_coffee.operates_in_basin,lifecycle,Source-backed effective period and exit conditions.,06-design-discussion-representation-catalog.md:735
backlog.catalog.7befccf4f0fb,institution.qa.energy_companies,owned_state,state.qa.energy_companies.commodities,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.7c6e130cb90b,institution.sa.energy_ministry,transmission,tx.economic_boundary.sa.energy_ministry.commodities.external_region.gulf,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.7d1d6dd1bbd2,UNKNOWN,owned_state,state.external_region.latin_america.currency,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.7da91e3d0dc8,inst.us.bank.burrow,relationship,rel.burrow.funding_counterparties,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",06-design-discussion-representation-catalog.md:872-876;06-design-discussion-representation-catalog.md:902
backlog.catalog.7dcc0dfb9261,firm.sa.saudi_aramco,transmission,tx.economic_boundary.sa.saudi_aramco.trade.external_region.gulf,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.7e606cdaface,institution.ru.security,owned_state,state.ru.security.trade,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.7eeeebfca76c,UNKNOWN,owned_state,state.external_region.east_asia.financial_stress,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.7ef6ee81ffd3,process.climate_agriculture.amazon_basin,transmission,tx.amazon_process.to_hog_operations,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",06-design-discussion-representation-catalog.md:652
backlog.catalog.7f2f52bb5061,institution.ae.central_bank,transmission,tx.economic_boundary.ae.central_bank.currency.external_region.gulf,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.7f56503dbef2,institution.us.new_york_fed,relationship,rel.publishes.us.new_york_fed.sofr,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",06-design-discussion-representation-catalog.md:1030-1042
backlog.catalog.7f99833db71f,mechanism.transform.global.timber_milling,relationship,rel.timber_milling.supplied_by_amazon_forestry,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",06-design-discussion-representation-catalog.md:738
backlog.catalog.7fb48c846d3d,UNKNOWN,transmission,tx.economic_boundary.external_region.eastern_europe.growth_demand.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.7fbc64c46716,UNKNOWN,owned_state,state.external_region.eastern_europe.dollar_funding,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.7fc03813835d,institution.sa.energy_ministry,owned_state,state.sa.energy_ministry.policy_stance,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.801f249f8295,mechanism.transform.global.hogs_slaughter,owned_state,queue.hogs_slaughter.work,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:612
backlog.catalog.803df24b5152,institution.eu.national_central_banks,owned_state,state.eu.national_central_banks.dollar_funding,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.80674d36e2a4,institution.media.gnbc,relationship,rel.operates.media.gnbc_company.gnbc,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",06-design-discussion-representation-catalog.md:371;05-design-discussion-representation-bible.md:1691
backlog.catalog.80d687a8b25f,UNKNOWN,owned_state,state.external_region.southeast_asia.financial_stress,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.8106736a493f,UNKNOWN,owned_state,state.external_region.middle_east.growth_demand,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.8189ea1c80d2,UNKNOWN,owned_state,state.external_region.gulf.growth_demand,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.822479b61afc,market.us.treasury.secondary,owned_state,state.treasury.secondary_clearing,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:334;06-design-discussion-representation-catalog.md:1132-1135
backlog.catalog.82436812408d,UNKNOWN,relationship,rel.aggregates_into.external_region.latin_america.global,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.83ae3a15bf83,UNKNOWN,owned_state,state.external_region.latin_america.financial_stress,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.845c5353d40d,institution.cn.pboc,owned_state,state.cn.pboc.policy_stance,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.8533eda7e220,institution.il.bank_of_israel,owned_state,state.il.bank_of_israel.currency,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.85ad6924b74f,outlet.media.loonberg,transmission,tx.media.loonberg.honkbox,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",task.md:818-826;05-design-discussion-representation-bible.md:184-185
backlog.catalog.85ee4c9b2164,UNKNOWN,transmission,tx.economic_boundary.external_region.gulf.dollar_funding.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.88712ad79259,region.sa.aggregate,entity_scope,region.sa.aggregate->adapter.external_region.latin_america,scope,Resolved typed scope endpoint.,06-design-discussion-representation-catalog.md:510
backlog.catalog.88d7d4154958,inst.us.bank.burrow,transmission,tx.burrow.orders.to_markets,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",06-design-discussion-representation-catalog.md:881-884;07-design-discussion-burrow-composition-probe.md:139
backlog.catalog.88fdfad3371f,institution.cn.party_leadership,owned_state,state.cn.party_leadership.trade,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.89251ccdf585,institution.uk.boe,owned_state,state.uk.boe.policy_stance,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.8942ab308ee1,institution.uk.government,transmission,tx.economic_boundary.uk.government.trade.external_region.aggregated_europe,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.8a4ba8bc0847,cohort.ru.energy_firms,action_domain,commodities,owner_or_content,Resolved owner and source-backed action contract.,04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.8b3cf4e44226,inst.us.bank.burrow,relationship,rel.burrow.supervised_by_state,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",None
backlog.catalog.8c7abf01e2c7,institution.sg.mas,transmission,tx.economic_boundary.sg.mas.currency.external_region.southeast_asia,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.8d3479fedfc3,institution.eu.national_central_banks,owned_state,state.eu.national_central_banks.financial_stress,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.8d46f8ecc0ed,UNKNOWN,owned_state,state.external_region.gulf.financial_stress,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.8dd8105c40d3,institution.ae.sovereign_funds,transmission,tx.economic_boundary.ae.sovereign_funds.reserves.external_region.gulf,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.8e0bcf37871c,UNKNOWN,transmission,tx.economic_boundary.external_region.gulf.currency.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.8e124a9d8872,institution.eu.commission,transmission,tx.economic_boundary.eu.commission.trade.external_region.aggregated_europe,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.8eb189b01006,network.media.the_herd,transmission,tx.media.the_herd.loonberg,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",task.md:818-826;05-design-discussion-representation-bible.md:184-185
backlog.catalog.8ef237b58303,mechanism.measurement.us.consumer_prices,owned_state,state.us_prices.sample,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:621
backlog.catalog.8f1dbfa0669a,institution.eu.member_governments,owned_state,state.eu.member_governments.financial_stress,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.8fbb03bdc518,UNKNOWN,owned_state,state.external_region.eastern_europe.currency,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.8fc372903744,market.us.treasury.auction,owned_state,state.treasury.auction_clearing,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:333
backlog.catalog.90e7eb8fc473,UNKNOWN,transmission,tx.economic_boundary.external_region.central_africa.commodities.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.90ec296beca7,institution.jp.boj,owned_state,state.jp.boj.policy_stance,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.91285c6e7596,UNKNOWN,transmission,tx.economic_boundary.external_region.aggregated_europe.reserves.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.913040c3d432,cohort.uk.pensions,transmission,tx.economic_boundary.uk.pensions.financial_stress.external_region.aggregated_europe,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.9138cce3406a,UNKNOWN,transmission,tx.supervision.to_burrow,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",06-design-discussion-representation-catalog.md:877-884;06-design-discussion-representation-catalog.md:904;07-design-discussion-burrow-composition-probe.md:184
backlog.catalog.91b31fca86d7,institution.sa.sama,owned_state,state.sa.sama.reserves,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.91b71774d52f,UNKNOWN,transmission,tx.economic_boundary.external_region.eastern_europe.financial_stress.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.91c0ebbaaec6,institution.eu.ecb,owned_state,state.eu.ecb.dollar_funding,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.92e09d7cf556,UNKNOWN,owned_state,state.external_region.east_asia.dollar_funding,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.93578b04335b,mechanism.flow.global.coffee,owned_state,queue.green_coffee_flow.delivery,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:614
backlog.catalog.93db068e45e2,UNKNOWN,transmission,tx.facility_offers.to_burrow,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",06-design-discussion-representation-catalog.md:877-884;06-design-discussion-representation-catalog.md:904;07-design-discussion-burrow-composition-probe.md:190
backlog.catalog.93de3016a097,institution.ru.presidency,owned_state,state.ru.presidency.trade,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.947e796ac208,institution.sa.sama,owned_state,state.sa.sama.currency,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.9636f1dae2dc,inst.us.bank.burrow,transmission,tx.burrow.reports_and_claims.to_information,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",None
backlog.catalog.964a1de965e3,UNKNOWN,owned_state,state.external_region.eastern_europe.commodities,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.976fe1d48255,market.product.global.coffee,owned_state,state.green_coffee_market.clearing,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:617
backlog.catalog.97917418ee15,UNKNOWN,relationship,rel.aggregates_into.external_region.east_asia.global,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.9894f0d98f65,UNKNOWN,owned_state,state.external_region.central_africa.reserves,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.98a724851756,industry.amazon_basin.coffee,owned_state,inventory.amazon_coffee.green_coffee,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:607
backlog.catalog.98b7ffa9ab91,cohort.jp.banks,owned_state,state.jp.banks.financial_stress,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.98e070b0752b,UNKNOWN,transmission,tx.economic_boundary.external_region.latin_america.reserves.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.998732a8d08d,institution.cn.safe,transmission,tx.economic_boundary.cn.safe.reserves.external_region.east_asia,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.9b30d0196083,firm.sa.saudi_aramco,transmission,tx.economic_boundary.sa.saudi_aramco.commodities.external_region.gulf,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.9b62a6eac639,stateful.us.treasury.duration_supply,owned_state,state.treasury.duration_supply,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:173-184;04-design-discussion-minimum-simulation-kernel.md:215
backlog.catalog.9b83632ad0df,institution.sa.sovereign_funds,owned_state,state.sa.sovereign_funds.reserves,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.9b897da70529,mechanism.cross_border.chokepoint_shipping,transmission,tx.economic_boundary.cross_border.chokepoint_shipping.commodities.external_region.middle_east,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.9bc827445a76,UNKNOWN,owned_state,state.external_region.middle_east.financial_stress,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.9be3f14eac95,market.product.global.timber,owned_state,state.lumber_market.residual,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:617
backlog.catalog.9cbceb492f6a,institution.qa.central_bank,owned_state,state.qa.central_bank.reserves,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.9cf43afca32e,inst.us.federal_reserve.new_york,relationship,rel.publication.nyfed_sofr,lifecycle,Source-backed effective period and exit conditions.,06-design-discussion-representation-catalog.md:1030-1043
backlog.catalog.9dbc0a895a6c,institution.cn.safe,transmission,tx.economic_boundary.cn.safe.currency.external_region.east_asia,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.9e1aa5eddbdb,UNKNOWN,transmission,tx.depositor_instructions.to_burrow,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",06-design-discussion-representation-catalog.md:877-884;06-design-discussion-representation-catalog.md:904;07-design-discussion-burrow-composition-probe.md:140;07-design-discussion-burrow-composition-probe.md:192
backlog.catalog.9e9a992d415a,body.us.federal_reserve.reserve_bank_boards,relationship,rel.governance.reserve_boards_other_banks,lifecycle,Source-backed effective period and exit conditions.,06-design-discussion-representation-catalog.md:257
backlog.catalog.9eb77046e6e3,legal.us.federal_reserve.section_13_3,transmission,tx.13_3.treasury.authorization,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",07-design-discussion-burrow-composition-probe.md:160-173
backlog.catalog.9f2458fd0a03,UNKNOWN,owned_state,state.external_region.latin_america.trade,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.9f5fd69e7cd8,UNKNOWN,transmission,tx.economic_boundary.external_region.aggregated_europe.currency.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.9f78b7c41def,institution.sg.mas,owned_state,state.sg.mas.currency,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.9faecd54c4f9,institution.qa.sovereign_funds,transmission,tx.economic_boundary.qa.sovereign_funds.dollar_funding.external_region.gulf,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.a03dc49e6da7,institution.us.community.reinvestment.advocates,relationship,rel.represents.community.reinvestment.advocates.low.income.underserved.borrowers,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",06-design-discussion-representation-catalog.md:310
backlog.catalog.a06849b6c994,institution.ir.central_bank,transmission,tx.economic_boundary.ir.central_bank.currency.external_region.middle_east,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.a098af626ce4,institution.ir.oil,owned_state,state.ir.oil.commodities,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.a0ad819f7f53,institution.fr.government,owned_state,state.fr.government.financial_stress,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.a0b2a1ca01d5,cohort.ru.energy_firms,transmission,tx.economic_boundary.ru.energy_firms.trade.external_region.eastern_europe,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.a17ebe90c868,institution.cn.party_leadership,transmission,tx.economic_boundary.cn.party_leadership.policy_stance.external_region.east_asia,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.a3fc4a992991,UNKNOWN,owned_state,state.external_region.east_asia.policy_stance,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.a40301cde8a0,UNKNOWN,owned_state,state.external_region.aggregated_europe.commodities,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.a5d8f13b3bd6,UNKNOWN,owned_state,state.external_region.east_asia.growth_demand,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.a647891a4be0,UNKNOWN,transmission,tx.economic_boundary.external_region.southeast_asia.growth_demand.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.a6ab0f1bf051,UNKNOWN,owned_state,state.external_region.central_africa.currency,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.a7bc3b6b5709,UNKNOWN,owned_state,state.external_region.middle_east.policy_stance,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.a873b2ddeec4,mechanism.us.ficc.sponsored_gcf_repo,owned_state,state.ficc.repo_clearing,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:337
backlog.catalog.a89b783124c8,inst.us.bank.burrow,relationship,rel.burrow.operates_in_home_district,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",06-design-discussion-representation-catalog.md:872-876;06-design-discussion-representation-catalog.md:902
backlog.catalog.a8afb0a57280,inst.us.bank.burrow,transmission,tx.burrow.requests.to_payments,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",06-design-discussion-representation-catalog.md:881-884;07-design-discussion-burrow-composition-probe.md:191-192
backlog.catalog.a8ed5e3e5efa,UNKNOWN,owned_state,state.external_region.gulf.currency,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.a90ef0d1e31f,UNKNOWN,owned_state,state.external_region.aggregated_europe.currency,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.a9350836f9dc,mechanism.measurement.us.consumer_prices,transmission,tx.measurement.us_consumer_prices.pce_reference,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",06-design-discussion-representation-catalog.md:523;06-design-discussion-representation-catalog.md:683;05-design-discussion-representation-bible.md:317
backlog.catalog.a94291c3e0ad,UNKNOWN,transmission,tx.economic_boundary.external_region.gulf.reserves.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.a96a51f68df9,UNKNOWN,transmission,tx.economic_boundary.external_region.latin_america.commodities.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.a9cbe888c402,institution.it.government,owned_state,state.it.government.financial_stress,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.aa628b193b2b,inst.us.bank.burrow,entity_scope,inst.us.bank.burrow->UNKNOWN,scope,Resolved typed scope endpoint.,06-design-discussion-representation-catalog.md:819; intended subject: region.us.home_district
backlog.catalog.aaab5869f838,process.climate_agriculture.amazon_basin,transmission,tx.amazon_process.to_coffee,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",06-design-discussion-representation-catalog.md:649
backlog.catalog.ab2062876b15,market.product.global.timber,owned_state,state.lumber_market.failure,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:617
backlog.catalog.ab232fa39e15,institution.jp.mof,transmission,tx.economic_boundary.jp.mof.currency.external_region.east_asia,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.abbec2a7f70e,institution.eu.national_central_banks,transmission,tx.economic_boundary.eu.national_central_banks.dollar_funding.external_region.aggregated_europe,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.abfdcfe7b11c,UNKNOWN,transmission,tx.economic_boundary.external_region.aggregated_europe.growth_demand.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.ad3f5ac5aa87,institution.sg.mas,owned_state,state.sg.mas.reserves,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.ad631e6fece9,institution.eu.ecb,transmission,tx.economic_boundary.eu.ecb.policy_stance.external_region.aggregated_europe,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.adccc3ea9c22,UNKNOWN,transmission,tx.economic_boundary.external_region.central_africa.reserves.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.ae4117859798,institution.il.bank_of_israel,owned_state,state.il.bank_of_israel.policy_stance,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.ae57e4c9292f,institution.ae.sovereign_funds,transmission,tx.economic_boundary.ae.sovereign_funds.dollar_funding.external_region.gulf,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.ae60507785a7,institution.cn.pboc,transmission,tx.economic_boundary.cn.pboc.policy_stance.external_region.east_asia,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.ae7c9daa0830,institution.sa.energy_ministry,owned_state,state.sa.energy_ministry.commodities,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.aeb97217756d,institution.uk.government,transmission,tx.economic_boundary.uk.government.policy_stance.external_region.aggregated_europe,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.aed18426a601,mechanism.measurement.us.consumer_prices,transmission,tx.measurement.us_consumer_prices.cpi_pce_references,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",06-design-discussion-representation-catalog.md:523;06-design-discussion-representation-catalog.md:683;05-design-discussion-representation-bible.md:317
backlog.catalog.af082c338b17,institution.eu.ecb,transmission,tx.economic_boundary.eu.ecb.currency.external_region.aggregated_europe,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.af5de4ecb970,UNKNOWN,transmission,tx.economic_boundary.external_region.latin_america.currency.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.af9e8fb4c49c,UNKNOWN,owned_state,state.external_region.central_africa.financial_stress,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.b03337b8c59c,legal.us.federal_reserve.section_13_3,relationship,rel.authority.13_3_treasury_consent,lifecycle,Source-backed effective period and exit conditions.,06-design-discussion-representation-catalog.md:1519-1521;07-design-discussion-burrow-composition-probe.md:160-173
backlog.catalog.b11e0c0d09c9,institution.it.government,transmission,tx.economic_boundary.it.government.financial_stress.external_region.aggregated_europe,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.b14a9ea885cd,mechanism.transform.global.timber_milling,owned_state,state.timber_milling.capacity,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:610
backlog.catalog.b19deb63d535,cohort.sg.dollar_funding_banks,owned_state,state.sg.dollar_funding_banks.financial_stress,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.b262fcc459d9,process.climate_agriculture.amazon_basin,owned_state,state.amazon_process.fire,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:605
backlog.catalog.b27d720c7e59,institution.us.realtors,relationship,rel.represents.realtors.real.estate.agents,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",06-design-discussion-representation-catalog.md:306
backlog.catalog.b2c05da02517,mechanism.flow.global.timber,owned_state,state.lumber_flow.capacity,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:613
backlog.catalog.b2d2876cda73,institution.sg.shipping,owned_state,state.sg.shipping.trade,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.b302678be405,record.media.the_herd.large_fox_shop_cutting_10s,relationship,rel.sources.media.herd_cutting_10s.the_herd,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",task.md:742;task.md:796-805
backlog.catalog.b4059bf5666a,record.media.loonberg.ust_10y_auction_tail,relationship,rel.sources.media.loonberg_auction_tail.loonberg,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",task.md:731;task.md:796-805
backlog.catalog.b4ec3fce1fa9,inst.us.bank.burrow,relationship,rel.burrow.potential_acquirers,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",06-design-discussion-representation-catalog.md:872-876;06-design-discussion-representation-catalog.md:902
backlog.catalog.b62d5c18968c,firm.sa.saudi_aramco,owned_state,state.sa.saudi_aramco.commodities,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.b7b4d89d82d2,institution.uk.boe,transmission,tx.economic_boundary.uk.boe.financial_stress.external_region.aggregated_europe,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.b881cf56ac6d,institution.us.securities.industry.association,relationship,rel.represents.securities.industry.association.dealers.market.makers,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",06-design-discussion-representation-catalog.md:304
backlog.catalog.b88534a75dcd,institution.ru.central_bank,transmission,tx.economic_boundary.ru.central_bank.policy_stance.external_region.eastern_europe,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.b89a66b2d038,inst.us.bank.burrow,relationship,rel.burrow.chartered_by,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",06-design-discussion-representation-catalog.md:858;06-design-discussion-representation-catalog.md:872-876;06-design-discussion-representation-catalog.md:902
backlog.catalog.b8a507882474,institution.jp.boj,owned_state,state.jp.boj.financial_stress,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.b8f18a4fadcc,industry.amazon_basin.coffee,owned_state,state.amazon_coffee.employment,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:607
backlog.catalog.b90c7e4921f6,industry.amazon_basin.pork,relationship,rel.amazon_pork.operates_in_basin,lifecycle,Source-backed effective period and exit conditions.,06-design-discussion-representation-catalog.md:737
backlog.catalog.b946a3e2645f,UNKNOWN,relationship,rel.aggregates_into.external_region.central_africa.global,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.b9d4bede5228,institution.il.bank_of_israel,transmission,tx.economic_boundary.il.bank_of_israel.policy_stance.external_region.middle_east,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.ba44fdc3b97a,institution.eu.ecb,owned_state,state.eu.ecb.financial_stress,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.ba63e808da73,body.il.cabinet,action_domain,policy_stance,owner_or_content,Resolved owner and source-backed action contract.,04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.bc5fa8a38e6a,institution.uk.government,owned_state,state.uk.government.trade,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.bc6648635a0e,institution.il.security,transmission,tx.economic_boundary.il.security.commodities.external_region.middle_east,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.bc8a7876bbbe,UNKNOWN,transmission,tx.economic_boundary.external_region.southeast_asia.policy_stance.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.bc9f7b4a453d,mechanism.flow.global.pork,owned_state,state.pork_flow.cold_chain,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:615
backlog.catalog.be65da2be424,institution.ru.presidency,transmission,tx.economic_boundary.ru.presidency.trade.external_region.eastern_europe,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.be835912b460,institution.sg.mas,owned_state,state.sg.mas.policy_stance,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.bf151f8ba107,UNKNOWN,transmission,tx.economic_boundary.external_region.southeast_asia.financial_stress.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.bfd93403f1d6,institution.ae.sovereign_funds,owned_state,state.ae.sovereign_funds.dollar_funding,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.c0ef289fbbe0,institution.cn.policy_banks,transmission,tx.economic_boundary.cn.policy_banks.dollar_funding.external_region.east_asia,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.c15bda8e892e,institution.ir.central_bank,owned_state,state.ir.central_bank.reserves,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.c16311b642b7,auth.us.primary_dealer_designation,relationship,rel.designation.primary_dealers,lifecycle,Source-backed effective period and exit conditions.,06-design-discussion-representation-catalog.md:278;06-design-discussion-representation-catalog.md:378
backlog.catalog.c2155ba030cd,facility.us.federal_reserve.srf,owned_state,state.srf.take_up,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:1005-1016
backlog.catalog.c22e04f5b718,industry.amazon_basin.forestry,relationship,rel.amazon_forestry.extraction_right,lifecycle,Source-backed effective period and exit conditions.,06-design-discussion-representation-catalog.md:648
backlog.catalog.c2399005809a,body.us.federal_reserve.fomc,transmission,tx.fomc.directive.ny_markets,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",07-design-discussion-burrow-composition-probe.md:144;07-design-discussion-burrow-composition-probe.md:150-153
backlog.catalog.c2a2c8283b32,UNKNOWN,transmission,tx.economic_boundary.external_region.east_asia.dollar_funding.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.c312994a5cf7,institution.ae.energy_companies,transmission,tx.economic_boundary.ae.energy_companies.commodities.external_region.gulf,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.c3197c907a25,institution.cn.pboc,owned_state,state.cn.pboc.currency,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.c32dc6838c36,institution.media.loonberg,relationship,rel.operates.media.loonberg_company.loonberg,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",06-design-discussion-representation-catalog.md:371;05-design-discussion-representation-bible.md:1691
backlog.catalog.c3303044dad8,UNKNOWN,transmission,tx.economic_boundary.external_region.central_africa.trade.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.c33a54682179,institution.sa.sovereign_funds,transmission,tx.economic_boundary.sa.sovereign_funds.dollar_funding.external_region.gulf,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.c53797b5e1c1,mechanism.measurement.us.consumer_prices,transmission,tx.us_prices.to_cpi_pce_references,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",06-design-discussion-representation-catalog.md:683
backlog.catalog.c54ecfcdb025,mechanism.transform.global.hogs_slaughter,owned_state,state.hogs_slaughter.capacity,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:612
backlog.catalog.c55dd836232c,institution.il.bank_of_israel,transmission,tx.economic_boundary.il.bank_of_israel.reserves.external_region.middle_east,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.c5ca4705f479,UNKNOWN,owned_state,state.external_region.southeast_asia.policy_stance,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.c65be03b57b1,institution.eu.ecb,owned_state,state.eu.ecb.policy_stance,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.c6a81adf76ee,UNKNOWN,owned_state,state.external_region.central_africa.commodities,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.c7188997220b,institution.ir.oil,owned_state,state.ir.oil.trade,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.c75498d88911,institution.eu.ecb,transmission,tx.economic_boundary.eu.ecb.dollar_funding.external_region.aggregated_europe,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.c779c6c57970,cohort.jp.pensions,owned_state,state.jp.pensions.financial_stress,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.c78df9b528da,cohort.jp.insurers,transmission,tx.economic_boundary.jp.insurers.reserves.external_region.east_asia,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.c96fe0ac5624,office.media.gnbc.anchor_chair,relationship,rel.belongs_to.media.gnbc_anchor_chair.gnbc,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",06-design-discussion-representation-catalog.md:1551;06-design-discussion-representation-catalog.md:1689
backlog.catalog.ca91dade88b7,UNKNOWN,transmission,tx.economic_boundary.external_region.eastern_europe.policy_stance.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.caad04c88469,institution.us.bls,relationship,rel.publishes.us.bls.cpi,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",06-design-discussion-representation-catalog.md:1043;05-design-discussion-representation-bible.md:183;05-design-discussion-representation-bible.md:317
backlog.catalog.caad73af5ebf,body.us.federal_reserve.fomc,relationship,rel.schedule.fomc,lifecycle,Source-backed effective period and exit conditions.,06-design-discussion-representation-catalog.md:377;07-design-discussion-burrow-composition-probe.md:171
backlog.catalog.cbed78de895d,UNKNOWN,owned_state,state.external_region.eastern_europe.financial_stress,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.cc2ff4a94094,institution.uk.dmo,owned_state,state.uk.dmo.financial_stress,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.ccb792bc3c37,institution.eu.member_governments,transmission,tx.economic_boundary.eu.member_governments.policy_stance.external_region.aggregated_europe,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.ccd08257a85c,institution.uk.dmo,owned_state,state.uk.dmo.dollar_funding,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.ccf8ad8d21f3,outlet.media.aftv,transmission,tx.media.aftv.mass_public_belief,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",task.md:820-826;04-design-discussion-minimum-simulation-kernel.md:52-56
backlog.catalog.cd783a5eb7aa,UNKNOWN,owned_state,state.external_region.eastern_europe.growth_demand,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.cdeb7ab92900,UNKNOWN,owned_state,state.external_region.middle_east.dollar_funding,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.ce21e7dd11a5,outlet.media.gnbc,transmission,tx.media.gnbc.mass_public_belief,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",task.md:820-826;04-design-discussion-minimum-simulation-kernel.md:52-56
backlog.catalog.ce6439debe54,UNKNOWN,transmission,tx.economic_boundary.external_region.gulf.trade.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.cfc9b3ae0322,firm.sa.saudi_aramco,owned_state,state.sa.saudi_aramco.trade,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.d04f409468f1,UNKNOWN,transmission,tx.economic_boundary.external_region.east_asia.reserves.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.d0ca9547f66e,UNKNOWN,transmission,tx.economic_boundary.external_region.latin_america.financial_stress.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.d15428d889a9,institution.qa.sovereign_funds,owned_state,state.qa.sovereign_funds.dollar_funding,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.d158ed16432a,industry.amazon_basin.pork,owned_state,state.amazon_hogs.employment,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:609
backlog.catalog.d18e54a8a769,market.product.global.pork,owned_state,state.pork_market.clearing,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:617
backlog.catalog.d1db33da0f90,agreement.us.repo.bilateral,owned_state,state.repo.contract_terms,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:335;06-design-discussion-representation-catalog.md:1182
backlog.catalog.d23f5cac54f1,industry.amazon_basin.pork,owned_state,inventory.amazon_hogs.live_hogs,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:609
backlog.catalog.d4c06c4ff24b,UNKNOWN,transmission,tx.economic_boundary.external_region.aggregated_europe.dollar_funding.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.d5405e379efe,institution.ae.energy_companies,owned_state,state.ae.energy_companies.trade,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.d5f49d76a902,market.product.global.timber,owned_state,state.lumber_market.orders,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:617
backlog.catalog.d6bc693a567a,process.climate_agriculture.amazon_basin,owned_state,state.amazon_process.recovery,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:605
backlog.catalog.d7971e578c2e,facility.us.federal_reserve.srf,owned_state,state.srf.terms,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:1005-1016
backlog.catalog.d845b6ed5017,UNKNOWN,owned_state,state.external_region.aggregated_europe.reserves,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.d87d5044e199,industry.amazon_basin.forestry,owned_state,account.amazon_forestry.finance,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:606
backlog.catalog.d97b7d34e9a2,UNKNOWN,transmission,tx.economic_boundary.external_region.central_africa.growth_demand.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.d97b8437c2db,institution.us.small.business.federation,relationship,rel.represents.small.business.federation.small.firms,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",06-design-discussion-representation-catalog.md:308
backlog.catalog.d99cf37009a9,UNKNOWN,owned_state,state.external_region.central_africa.trade,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.da2687b3baa6,cohort.eu.exposed_banks,transmission,tx.economic_boundary.eu.exposed_banks.financial_stress.external_region.aggregated_europe,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.da40953c9694,cohort.jp.pensions,transmission,tx.economic_boundary.jp.pensions.reserves.external_region.east_asia,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.dac7beb7bc21,inst.us.treasury,transmission,tx.issuance.treasury.duration_supply,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:173-184;04-design-discussion-minimum-simulation-kernel.md:215
backlog.catalog.daf4c2e76361,record.media.goosetogetherstrong.margin_call,relationship,rel.sources.media.goose_margin_call.goosetogetherstrong,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",task.md:775;task.md:796-805
backlog.catalog.db61988e5d2b,institution.sg.sovereign_funds,transmission,tx.economic_boundary.sg.sovereign_funds.dollar_funding.external_region.southeast_asia,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.db67b3334ef4,institution.qa.central_bank,owned_state,state.qa.central_bank.currency,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.dcdd2021c49b,mechanism.flow.global.pork,owned_state,state.pork_flow.capacity,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:615
backlog.catalog.ddc357dedd95,institution.cn.military,owned_state,state.cn.military.trade,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.dded9846f411,cohort.jp.banks,transmission,tx.economic_boundary.jp.banks.dollar_funding.external_region.east_asia,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.de1b1459e72f,UNKNOWN,transmission,tx.economic_boundary.external_region.eastern_europe.commodities.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.de63f11cf8c3,inst.us.bank.burrow,transmission,tx.burrow.requests.to_facilities,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",06-design-discussion-representation-catalog.md:881-884;07-design-discussion-burrow-composition-probe.md:191;07-design-discussion-burrow-composition-probe.md:311
backlog.catalog.df4c22d8a0f4,mechanism.transform.global.hogs_slaughter,relationship,rel.hogs_slaughter.supplied_by_amazon_hogs,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",06-design-discussion-representation-catalog.md:739
backlog.catalog.e08abee363e2,process.climate_agriculture.amazon_basin,owned_state,inventory.amazon_process.standing_biomass,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:605
backlog.catalog.e0af49fdf7d0,inst.us.treasury,owned_state,state.treasury.issuance_schedule,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:215
backlog.catalog.e0b36e7ef7bf,UNKNOWN,owned_state,state.external_region.east_asia.commodities,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.e1052b9ea240,body.us.federal_reserve.fomc,relationship,rel.governance.fomc_federal_reserve,lifecycle,Source-backed effective period and exit conditions.,06-design-discussion-representation-catalog.md:255;06-design-discussion-representation-catalog.md:269
backlog.catalog.e127bba39019,institution.cn.policy_banks,transmission,tx.economic_boundary.cn.policy_banks.financial_stress.external_region.east_asia,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.e1385163547d,institution.us.retiree.association.aarp.analogue,relationship,rel.represents.retiree.association.near.retirees,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",06-design-discussion-representation-catalog.md:307
backlog.catalog.e1d6dc96cf53,institution.sg.mas,transmission,tx.economic_boundary.sg.mas.reserves.external_region.southeast_asia,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.e23e9bc7866a,UNKNOWN,owned_state,state.external_region.aggregated_europe.financial_stress,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.e2506dcd051f,adapter.input.amazon_basin.feed_grain,owned_state,inventory.amazon_feed.feed_grain,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:608
backlog.catalog.e265883c2526,institution.ir.oil,transmission,tx.economic_boundary.ir.oil.commodities.external_region.middle_east,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.e284f6c2bca0,UNKNOWN,owned_state,state.external_region.east_asia.reserves,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.e361fbe90a28,institution.ru.central_bank,owned_state,state.ru.central_bank.policy_stance,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.e42eb037a5df,cohort.uk.pensions,owned_state,state.uk.pensions.financial_stress,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.e50acfc8ecbb,mechanism.measurement.us.consumer_prices,entity_scope,mechanism.measurement.us.consumer_prices->UNKNOWN,scope,Resolved typed scope endpoint.,06-design-discussion-representation-catalog.md:523
backlog.catalog.e526c2299b00,institution.cn.party_leadership,transmission,tx.economic_boundary.cn.party_leadership.trade.external_region.east_asia,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.e5b88e50df21,adapter.input.amazon_basin.feed_grain,owned_state,account.amazon_feed.external_source,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:608
backlog.catalog.e652959329a5,institution.eu.member_governments,transmission,tx.economic_boundary.eu.member_governments.financial_stress.external_region.aggregated_europe,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.e6a6071a488c,adapter.real.us.food_baskets,owned_state,state.us_food.roasted_coffee_allocation,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:619
backlog.catalog.e7070fc9d926,UNKNOWN,transmission,tx.economic_boundary.external_region.middle_east.financial_stress.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.e737f1ade45c,cohort.sg.dollar_funding_banks,transmission,tx.economic_boundary.sg.dollar_funding_banks.dollar_funding.external_region.southeast_asia,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.e73dd8160c22,institution.eu.commission,owned_state,state.eu.commission.policy_stance,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.e752e5605d52,institution.jp.boj,owned_state,state.jp.boj.reserves,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.e76bbf6e2e47,institution.de.government,owned_state,state.de.government.financial_stress,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.e83f4e9bbcc4,institution.qa.sovereign_funds,owned_state,state.qa.sovereign_funds.reserves,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.e8b757debd3b,institution.eu.ecb,owned_state,state.eu.ecb.currency,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.e8f4c7997de9,industry.amazon_basin.coffee,owned_state,state.amazon_coffee.crop,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:607
backlog.catalog.e98b45eeb7a7,institution.jp.mof,owned_state,state.jp.mof.reserves,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.ea1a394a2e54,institution.qa.central_bank,transmission,tx.economic_boundary.qa.central_bank.reserves.external_region.gulf,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.ea4e02eb7982,institution.us.managed.funds.association,relationship,rel.represents.managed.funds.association.hedge.funds,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",06-design-discussion-representation-catalog.md:305
backlog.catalog.eaab1032a5d8,institution.qa.sovereign_funds,transmission,tx.economic_boundary.qa.sovereign_funds.reserves.external_region.gulf,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.eac4eed9fa38,UNKNOWN,transmission,tx.economic_boundary.external_region.east_asia.policy_stance.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.eb4de26f55fb,person.media.wool_street_journal_fed_reporter,relationship,rel.holds.media.wsj_fed_reporter.wsj_fed_beat,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",06-design-discussion-representation-catalog.md:252;06-design-discussion-representation-catalog.md:1551-1558
backlog.catalog.eb898621447a,institution.sa.sovereign_funds,transmission,tx.economic_boundary.sa.sovereign_funds.reserves.external_region.gulf,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.ebe305265ed2,institution.us.labor.federation,relationship,rel.represents.labor.federation.workers.by.sector,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",06-design-discussion-representation-catalog.md:309
backlog.catalog.ebeef7819b35,institution.il.security,owned_state,state.il.security.trade,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.ec2026343567,UNKNOWN,transmission,tx.economic_boundary.external_region.central_africa.currency.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.ec7bd8389bdd,institution.us.retiree.association.aarp.analogue,relationship,rel.represents.retiree.association.retirees,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",06-design-discussion-representation-catalog.md:307
backlog.catalog.ecc06dcb7b00,institution.uk.government,owned_state,state.uk.government.policy_stance,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.ed0137e9f663,mechanism.transform.global.timber_milling,owned_state,queue.timber_milling.work,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:610
backlog.catalog.ed270b2ae305,UNKNOWN,transmission,tx.economic_boundary.external_region.southeast_asia.commodities.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.ed3bfb599d7b,institution.cn.state_council,transmission,tx.economic_boundary.cn.state_council.policy_stance.external_region.east_asia,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.ed81e7b057e7,market.us.treasury.secondary,transmission,tx.secondary.treasury.dealer_inventory,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:174-180
backlog.catalog.ee11a5641f91,person.media.gnbc_anchor,relationship,rel.holds.media.gnbc_anchor.gnbc_anchor_chair,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",06-design-discussion-representation-catalog.md:252;06-design-discussion-representation-catalog.md:1551-1558
backlog.catalog.ee79142570f9,body.il.cabinet,owned_state,state.il.cabinet.policy_stance,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.ef9bf9f67f41,institution.cn.pboc,transmission,tx.economic_boundary.cn.pboc.dollar_funding.external_region.east_asia,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.efdfca7eb86c,office.media.wool_street_journal.fed_beat,relationship,rel.belongs_to.media.wsj_fed_beat.wsj,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",06-design-discussion-representation-catalog.md:1551;06-design-discussion-representation-catalog.md:1689
backlog.catalog.f204f19c8265,UNKNOWN,owned_state,state.external_region.latin_america.dollar_funding,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.f20c181faa81,cohort.eu.exposed_banks,owned_state,state.eu.exposed_banks.dollar_funding,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.f20c90de7135,inst.us.federal_reserve.new_york,relationship,rel.membership.federal_reserve.new_york_system,lifecycle,Source-backed effective period and exit conditions.,06-design-discussion-representation-catalog.md:269;06-design-discussion-representation-catalog.md:960
backlog.catalog.f2471c009d68,UNKNOWN,transmission,tx.economic_boundary.external_region.east_asia.growth_demand.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.f26a367ed0f2,body.us.federal_reserve.fomc,relationship,rel.authority.fomc_srf_terms,lifecycle,Source-backed effective period and exit conditions.,06-design-discussion-representation-catalog.md:1008-1015
backlog.catalog.f276387c3f24,mechanism.transform.global.timber_milling,owned_state,state.timber_milling.losses,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:610
backlog.catalog.f36a4bf7113f,institution.uk.boe,owned_state,state.uk.boe.currency,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.f3c599065bdb,industry.amazon_basin.forestry,owned_state,state.amazon_forestry.employment,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:606
backlog.catalog.f46d1dcf0321,market.product.global.coffee,owned_state,state.green_coffee_market.failure,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:617
backlog.catalog.f4798c00a54c,inst.us.bank.burrow,relationship,rel.burrow.customer_accounts,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",06-design-discussion-representation-catalog.md:872-876;06-design-discussion-representation-catalog.md:902
backlog.catalog.f4805fa13d04,mechanism.transform.global.hogs_slaughter,owned_state,state.hogs_slaughter.cold_chain,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:612
backlog.catalog.f5749a273ca2,UNKNOWN,transmission,tx.economic_boundary.external_region.southeast_asia.dollar_funding.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.f5c3448ea0cf,mechanism.flow.global.coffee,owned_state,state.green_coffee_flow.capacity,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:614
backlog.catalog.f5d64fb28814,institution.il.bank_of_israel,owned_state,state.il.bank_of_israel.reserves,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.f67016965e61,institution.jp.boj,transmission,tx.economic_boundary.jp.boj.reserves.external_region.east_asia,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.f6a1a1be5cc3,UNKNOWN,transmission,tx.economic_boundary.external_region.central_africa.financial_stress.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.f765bd66de7a,industry.amazon_basin.pork,owned_state,inventory.amazon_hogs.feed_grain,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:609
backlog.catalog.f847475b92b0,institution.cn.safe,owned_state,state.cn.safe.currency,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.f8afd2683a13,institution.eu.commission,transmission,tx.economic_boundary.eu.commission.policy_stance.external_region.aggregated_europe,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.f90548317284,institution.uk.boe,owned_state,state.uk.boe.dollar_funding,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.f96f1943ded0,institution.eu.commission,owned_state,state.eu.commission.trade,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.f9a61dbb663f,mechanism.flow.global.pork,owned_state,queue.pork_flow.delivery,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:615
backlog.catalog.fa2f91a0c908,institution.us.farm.bureau.analogue,relationship,rel.represents.farm.bureau.analogue.agricultural.producers,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",06-design-discussion-representation-catalog.md:311
backlog.catalog.fafc88578715,UNKNOWN,owned_state,state.external_region.middle_east.commodities,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.fc52a438d763,institution.sg.shipping,transmission,tx.economic_boundary.sg.shipping.trade.external_region.southeast_asia,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.fc7d20d67192,institution.uk.dmo,transmission,tx.economic_boundary.uk.dmo.dollar_funding.external_region.aggregated_europe,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.fcc3d68a24ee,institution.uk.boe,transmission,tx.economic_boundary.uk.boe.currency.external_region.aggregated_europe,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.fd9ada22bd78,institution.cn.state_council,owned_state,state.cn.state_council.trade,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.fdbd5724db88,adapter.real.us.food_baskets,owned_state,account.us_food.external,state_transition,"Source-backed transition, witness, owner, and conserved unit.",06-design-discussion-representation-catalog.md:619
backlog.catalog.fe7967aa889a,inst.us.ficc,relationship,rel.operation.ficc_repo_clearer,lifecycle,Source-backed effective period and exit conditions.,06-design-discussion-representation-catalog.md:337
backlog.catalog.ff8a8c021780,institution.cn.policy_banks,owned_state,state.cn.policy_banks.financial_stress,state_transition,"Source-backed transition, witness, owner, and conserved unit.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.ff911642ebb8,UNKNOWN,transmission,tx.economic_boundary.external_region.east_asia.financial_stress.external_region.global,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.ffa291c07a37,institution.fr.government,transmission,tx.economic_boundary.fr.government.financial_stress.external_region.aggregated_europe,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
backlog.catalog.ffc48554457f,institution.us.investment.company.institute.analogue,relationship,rel.represents.investment.company.institute.analogue.money.market.mutual.funds,relationship,"Resolved subject, object, canonical owner, lifecycle, observability, and witness.",06-design-discussion-representation-catalog.md:302
backlog.catalog.fffdc5145cf5,institution.jp.boj,transmission,tx.economic_boundary.jp.boj.financial_stress.external_region.east_asia,endpoint,"Resolved producer, output/state, consumer, transformation owner, unit, witness, and fallback behavior.",04-design-discussion-minimum-simulation-kernel.md:819-837;05-design-discussion-representation-bible.md:678-690;05-design-discussion-representation-bible.md:737-771
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/closure/scenario_availability.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/closure/scenario_availability.csv
size_bytes: 101
mode_octal: "0644"
modified_at_utc: 2026-09-04T04:06:00.127397Z
sha256: c82bfdda600a910397fbfc7553a266b960acc0de9727f217aa9e6f9279a7078a
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
scenario_id,catalog_id,availability,selected_fidelity,provider_entry_id,uncertainty_notes,provenance
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/closure/scenario_candidates.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/closure/scenario_candidates.csv
size_bytes: 62
mode_octal: "0644"
modified_at_utc: 2026-09-04T04:06:00.127988Z
sha256: 66c653ad1fa4db9500daf78f765d5c5d1ce09c1784f2851c288e4a32c393a3b4
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
scenario_id,catalog_id,relevance,uncertainty_notes,provenance
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/closure/transmission_scenarios.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/closure/transmission_scenarios.csv
size_bytes: 52
mode_octal: "0644"
modified_at_utc: 2026-09-04T04:06:00.127619Z
sha256: bc895c16c7a777914529b24bcde07b910f00719b2396264f0755fb4e45b2fdcc
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
transmission_id,scenario_id,availability,provenance
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/closure/transmissions.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/closure/transmissions.csv
size_bytes: 45142
mode_octal: "0644"
modified_at_utc: 2026-09-04T18:32:41.158562Z
sha256: 8a2c972e96d22217ec406d86990cce7775adc6b15657dcec5e041a031f8427a7
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
transmission_id,producing_entry_id,producing_state_or_output,consuming_entry_id,consuming_input,payload_kind,value_domain,unit,direction,transformation_owner_id,effective_delay,persistence_or_expiry,capacity_ref,witness_kind,fallback_behavior,uncertainty_notes,provenance
tx.amazon_coffee.to_green_flow,industry.amazon_basin.coffee,output.amazon_coffee.green_coffee_offer,mechanism.flow.global.coffee,input.green_coffee_flow.supply_offer,quantity,GREEN_COFFEE by grade origin crop year title contract and window,mass,industry_to_flow,mechanism.flow.global.coffee,contract/delivery delay,expires by contract,state.green_coffee_flow.capacity,harvest inventory-release contract queue title/custody witness,flow adapter fallback,Exact counterparty UNKNOWN,06-design-discussion-representation-catalog.md:655
tx.amazon_forestry.to_milling,industry.amazon_basin.forestry,output.amazon_forestry.timber_offer,mechanism.transform.global.timber_milling,input.timber_milling.timber_offer,quantity,TIMBER by grade origin title contract and window,standardized solid cubic meter,industry_to_transformation,mechanism.transform.global.timber_milling,contract/delivery delay,expires by contract,state.timber_milling.capacity,harvest-right inventory-release contract queue title/custody witness,milling adapter fallback,Exact counterparty UNKNOWN,06-design-discussion-representation-catalog.md:653
tx.amazon_hogs.to_slaughter,industry.amazon_basin.pork,output.amazon_hogs.live_offer,mechanism.transform.global.hogs_slaughter,input.hogs_slaughter.live_offer,quantity,LIVE_HOGS with weight/health distribution origin title contract and window,headcount,industry_to_transformation,mechanism.transform.global.hogs_slaughter,contract/transport delay,expires by contract,state.hogs_slaughter.capacity,biological inventory conversion contract transport acceptance custody witness,slaughter adapter fallback,Exact counterparty UNKNOWN,06-design-discussion-representation-catalog.md:656
tx.amazon_process.biomass_to_forestry,process.climate_agriculture.amazon_basin,output.amazon_process.timber_release,industry.amazon_basin.forestry,input.amazon_forestry.timber_release,quantity,TIMBER with origin grade title concession and extraction witness,standardized solid cubic meter,process_to_industry,industry.amazon_basin.forestry,extraction and custody-transfer delay,titled inventory persists,state.amazon_forestry.capacity,extraction-right natural-stock title/custody delivery witness,process adapter preserves release interface,Exact delay UNKNOWN,06-design-discussion-representation-catalog.md:648
tx.coffee_roasting.request_to_demand,mechanism.transform.global.coffee_roasting,output.coffee_roasting.green_request,mechanism.demand.global.coffee,input.green_coffee_demand.participant_request,distribution,GREEN_COFFEE desired quantity by bid price grade origin and window,mass,transformation_to_demand,mechanism.demand.global.coffee,decision/aggregation delay,expires with market window,request-queue capacity,roaster plan budget request queue aggregation submission witness,absent when exogenous demand adapter selected,Exact budget UNKNOWN,06-design-discussion-representation-catalog.md:675
tx.external.dollar_funding_fx.euro_area.dollar_funding.capacity,adapter.external.euro_area,interface.dollar_funding.capacity,market.global.fx_spot_forward_basis,interface.dollar_funding.capacity,capacity,USD notional,USD_notional,inbound_to_us,adapter.external.euro_area,profile-period interface contract,Expires with the provider binding or profile period.,NONE,typed market interface registration,fall back to adapter.external.external_world,No domestic clearing price or macro consequence is owned by the provider.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
tx.external.dollar_funding_fx.euro_area.fx.condition,adapter.external.euro_area,interface.fx.condition,market.global.fx_spot_forward_basis,interface.fx.condition,observation,currency-pair quote,currency_pair_quote,inbound_to_us,adapter.external.euro_area,profile-period interface contract,Expires with the provider binding or profile period.,NONE,typed market interface registration,fall back to adapter.external.external_world,No domestic clearing price or macro consequence is owned by the provider.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
tx.external.dollar_funding_fx.external_world.dollar_funding.capacity,adapter.external.external_world,interface.dollar_funding.capacity,market.global.fx_spot_forward_basis,interface.dollar_funding.capacity,capacity,USD notional,USD_notional,inbound_to_us,adapter.external.external_world,profile-period interface contract,Expires with the provider binding or profile period.,NONE,typed market interface registration,terminal exogenous input,No domestic clearing price or macro consequence is owned by the provider.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
tx.external.dollar_funding_fx.external_world.fx.condition,adapter.external.external_world,interface.fx.condition,market.global.fx_spot_forward_basis,interface.fx.condition,observation,currency-pair quote,currency_pair_quote,inbound_to_us,adapter.external.external_world,profile-period interface contract,Expires with the provider binding or profile period.,NONE,typed market interface registration,terminal exogenous input,No domestic clearing price or macro consequence is owned by the provider.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
tx.external.dollar_funding_fx.japan.dollar_funding.capacity,adapter.external.japan,interface.dollar_funding.capacity,market.global.fx_spot_forward_basis,interface.dollar_funding.capacity,capacity,USD notional,USD_notional,inbound_to_us,adapter.external.japan,profile-period interface contract,Expires with the provider binding or profile period.,NONE,typed market interface registration,fall back to adapter.external.external_world,No domestic clearing price or macro consequence is owned by the provider.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
tx.external.dollar_funding_fx.japan.fx.condition,adapter.external.japan,interface.fx.condition,market.global.fx_spot_forward_basis,interface.fx.condition,observation,currency-pair quote,currency_pair_quote,inbound_to_us,adapter.external.japan,profile-period interface contract,Expires with the provider binding or profile period.,NONE,typed market interface registration,fall back to adapter.external.external_world,No domestic clearing price or macro consequence is owned by the provider.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
tx.external.dollar_funding_fx.offshore_asia.dollar_funding.capacity,adapter.external.offshore_asia,interface.dollar_funding.capacity,market.global.fx_spot_forward_basis,interface.dollar_funding.capacity,capacity,USD notional,USD_notional,inbound_to_us,adapter.external.offshore_asia,profile-period interface contract,Expires with the provider binding or profile period.,NONE,typed market interface registration,fall back to adapter.external.external_world,No domestic clearing price or macro consequence is owned by the provider.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
tx.external.dollar_funding_fx.offshore_asia.fx.condition,adapter.external.offshore_asia,interface.fx.condition,market.global.fx_spot_forward_basis,interface.fx.condition,observation,currency-pair quote,currency_pair_quote,inbound_to_us,adapter.external.offshore_asia,profile-period interface contract,Expires with the provider binding or profile period.,NONE,typed market interface registration,fall back to adapter.external.external_world,No domestic clearing price or macro consequence is owned by the provider.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
tx.external.dollar_funding_fx.united_kingdom.dollar_funding.capacity,adapter.external.united_kingdom,interface.dollar_funding.capacity,market.global.fx_spot_forward_basis,interface.dollar_funding.capacity,capacity,USD notional,USD_notional,inbound_to_us,adapter.external.united_kingdom,profile-period interface contract,Expires with the provider binding or profile period.,NONE,typed market interface registration,fall back to adapter.external.external_world,No domestic clearing price or macro consequence is owned by the provider.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
tx.external.dollar_funding_fx.united_kingdom.fx.condition,adapter.external.united_kingdom,interface.fx.condition,market.global.fx_spot_forward_basis,interface.fx.condition,observation,currency-pair quote,currency_pair_quote,inbound_to_us,adapter.external.united_kingdom,profile-period interface contract,Expires with the provider binding or profile period.,NONE,typed market interface registration,fall back to adapter.external.external_world,No domestic clearing price or macro consequence is owned by the provider.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
tx.external.energy_supply.external_energy.energy_supply.product_schedule,adapter.external.external_energy,interface.energy_supply.product_schedule,mechanism.us.external_market_inputs,interface.energy_supply.product_schedule,schedule,canonical product quantity per period,barrel,inbound_to_us,adapter.external.external_energy,profile-period interface contract,Expires with the provider binding or profile period.,NONE,typed market interface registration,terminal exogenous input,No domestic clearing price or macro consequence is owned by the provider.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
tx.external.energy_supply.gulf_exporters.energy_supply.product_schedule,adapter.external.gulf_exporters,interface.energy_supply.product_schedule,mechanism.us.external_market_inputs,interface.energy_supply.product_schedule,schedule,canonical product quantity per period,barrel,inbound_to_us,adapter.external.gulf_exporters,profile-period interface contract,Expires with the provider binding or profile period.,NONE,typed market interface registration,fall back to adapter.external.external_energy,No domestic clearing price or macro consequence is owned by the provider.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
tx.external.energy_supply.russia.energy_supply.product_schedule,adapter.external.russia,interface.energy_supply.product_schedule,mechanism.us.external_market_inputs,interface.energy_supply.product_schedule,schedule,canonical product quantity per period,barrel,inbound_to_us,adapter.external.russia,profile-period interface contract,Expires with the provider binding or profile period.,NONE,typed market interface registration,fall back to adapter.external.external_energy,No domestic clearing price or macro consequence is owned by the provider.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
tx.external.external_demand.external_world.external_demand.index,adapter.external.external_world,interface.external_demand.index,mechanism.us.external_market_inputs,interface.external_demand.index,observation,dimensionless demand index,dimensionless,inbound_to_us,adapter.external.external_world,profile-period interface contract,Expires with the provider binding or profile period.,NONE,typed market interface registration,terminal exogenous input,No domestic clearing price or macro consequence is owned by the provider.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
tx.external.external_demand.external_world.external_policy.condition,adapter.external.external_world,interface.external_policy.condition,mechanism.us.external_market_inputs,interface.external_policy.condition,observation,policy-state enum,policy_state_enum,inbound_to_us,adapter.external.external_world,profile-period interface contract,Expires with the provider binding or profile period.,NONE,typed market interface registration,terminal exogenous input,No domestic clearing price or macro consequence is owned by the provider.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
tx.external.foreign_financial_stress.emerging_markets.foreign_financial_stress.index,adapter.external.emerging_markets,interface.foreign_financial_stress.index,mechanism.us.external_market_inputs,interface.foreign_financial_stress.index,observation,dimensionless stress index,dimensionless,inbound_to_us,adapter.external.emerging_markets,profile-period interface contract,Expires with the provider binding or profile period.,NONE,typed market interface registration,fall back to adapter.external.external_world,No domestic clearing price or macro consequence is owned by the provider.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
tx.external.foreign_financial_stress.euro_area.foreign_financial_stress.index,adapter.external.euro_area,interface.foreign_financial_stress.index,mechanism.us.external_market_inputs,interface.foreign_financial_stress.index,observation,dimensionless stress index,dimensionless,inbound_to_us,adapter.external.euro_area,profile-period interface contract,Expires with the provider binding or profile period.,NONE,typed market interface registration,fall back to adapter.external.external_world,No domestic clearing price or macro consequence is owned by the provider.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
tx.external.foreign_financial_stress.external_world.foreign_financial_stress.index,adapter.external.external_world,interface.foreign_financial_stress.index,mechanism.us.external_market_inputs,interface.foreign_financial_stress.index,observation,dimensionless stress index,dimensionless,inbound_to_us,adapter.external.external_world,profile-period interface contract,Expires with the provider binding or profile period.,NONE,typed market interface registration,terminal exogenous input,No domestic clearing price or macro consequence is owned by the provider.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
tx.external.foreign_financial_stress.japan.foreign_financial_stress.index,adapter.external.japan,interface.foreign_financial_stress.index,mechanism.us.external_market_inputs,interface.foreign_financial_stress.index,observation,dimensionless stress index,dimensionless,inbound_to_us,adapter.external.japan,profile-period interface contract,Expires with the provider binding or profile period.,NONE,typed market interface registration,fall back to adapter.external.external_world,No domestic clearing price or macro consequence is owned by the provider.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
tx.external.foreign_financial_stress.united_kingdom.foreign_financial_stress.index,adapter.external.united_kingdom,interface.foreign_financial_stress.index,mechanism.us.external_market_inputs,interface.foreign_financial_stress.index,observation,dimensionless stress index,dimensionless,inbound_to_us,adapter.external.united_kingdom,profile-period interface contract,Expires with the provider binding or profile period.,NONE,typed market interface registration,fall back to adapter.external.external_world,No domestic clearing price or macro consequence is owned by the provider.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
tx.external.freight_shipping.freight_global.freight.capacity,adapter.external.freight_global,interface.freight.capacity,mechanism.us.external_market_inputs,interface.freight.capacity,capacity,canonical product quantity per period,standardized solid cubic meter,inbound_to_us,adapter.external.freight_global,profile-period interface contract,Expires with the provider binding or profile period.,NONE,typed market interface registration,terminal exogenous input,No domestic clearing price or macro consequence is owned by the provider.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
tx.external.freight_shipping.freight_global.freight.delay,adapter.external.freight_global,interface.freight.delay,mechanism.us.external_market_inputs,interface.freight.delay,delay,time,time_period,inbound_to_us,adapter.external.freight_global,profile-period interface contract,Expires with the provider binding or profile period.,NONE,typed market interface registration,terminal exogenous input,No domestic clearing price or macro consequence is owned by the provider.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
tx.external.freight_shipping.freight_gulf_suez.freight.capacity,adapter.external.freight_gulf_suez,interface.freight.capacity,mechanism.us.external_market_inputs,interface.freight.capacity,capacity,canonical product quantity per period,standardized solid cubic meter,inbound_to_us,adapter.external.freight_gulf_suez,profile-period interface contract,Expires with the provider binding or profile period.,NONE,typed market interface registration,fall back to adapter.external.freight_global,No domestic clearing price or macro consequence is owned by the provider.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
tx.external.freight_shipping.freight_gulf_suez.freight.delay,adapter.external.freight_gulf_suez,interface.freight.delay,mechanism.us.external_market_inputs,interface.freight.delay,delay,time,time_period,inbound_to_us,adapter.external.freight_gulf_suez,profile-period interface contract,Expires with the provider binding or profile period.,NONE,typed market interface registration,fall back to adapter.external.freight_global,No domestic clearing price or macro consequence is owned by the provider.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
tx.external.import_supply.external_world.import_supply.index,adapter.external.external_world,interface.import_supply.index,mechanism.us.external_market_inputs,interface.import_supply.index,observation,dimensionless supply index,dimensionless,inbound_to_us,adapter.external.external_world,profile-period interface contract,Expires with the provider binding or profile period.,NONE,typed market interface registration,terminal exogenous input,No domestic clearing price or macro consequence is owned by the provider.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
tx.external.import_supply.external_world.import_supply.product_schedule,adapter.external.external_world,interface.import_supply.product_schedule,mechanism.us.external_market_inputs,interface.import_supply.product_schedule,schedule,canonical product quantity per period,standardized solid cubic meter,inbound_to_us,adapter.external.external_world,profile-period interface contract,Expires with the provider binding or profile period.,NONE,typed market interface registration,terminal exogenous input,No domestic clearing price or macro consequence is owned by the provider.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
tx.external.us_duration_demand.china.foreign_reserves.flow,adapter.external.china,interface.foreign_reserves.flow,market.us.treasury.secondary,interface.foreign_reserves.flow,quantity,USD notional per period,USD_notional_per_period,inbound_to_us,adapter.external.china,profile-period interface contract,Expires with the provider binding or profile period.,NONE,typed market interface registration,fall back to adapter.external.external_world,No domestic clearing price or macro consequence is owned by the provider.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
tx.external.us_duration_demand.china.us_duration.demand_schedule,adapter.external.china,interface.us_duration.demand_schedule,market.us.treasury.secondary,interface.us_duration.demand_schedule,schedule,USD notional by instrument bucket,USD_notional,inbound_to_us,adapter.external.china,profile-period interface contract,Expires with the provider binding or profile period.,NONE,typed market interface registration,fall back to adapter.external.external_world,No domestic clearing price or macro consequence is owned by the provider.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
tx.external.us_duration_demand.external_world.foreign_reserves.flow,adapter.external.external_world,interface.foreign_reserves.flow,market.us.treasury.secondary,interface.foreign_reserves.flow,quantity,USD notional per period,USD_notional_per_period,inbound_to_us,adapter.external.external_world,profile-period interface contract,Expires with the provider binding or profile period.,NONE,typed market interface registration,terminal exogenous input,No domestic clearing price or macro consequence is owned by the provider.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
tx.external.us_duration_demand.external_world.us_duration.demand_schedule,adapter.external.external_world,interface.us_duration.demand_schedule,market.us.treasury.secondary,interface.us_duration.demand_schedule,schedule,USD notional by instrument bucket,USD_notional,inbound_to_us,adapter.external.external_world,profile-period interface contract,Expires with the provider binding or profile period.,NONE,typed market interface registration,terminal exogenous input,No domestic clearing price or macro consequence is owned by the provider.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
tx.external.us_duration_demand.japan.foreign_reserves.flow,adapter.external.japan,interface.foreign_reserves.flow,market.us.treasury.secondary,interface.foreign_reserves.flow,quantity,USD notional per period,USD_notional_per_period,inbound_to_us,adapter.external.japan,profile-period interface contract,Expires with the provider binding or profile period.,NONE,typed market interface registration,fall back to adapter.external.external_world,No domestic clearing price or macro consequence is owned by the provider.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
tx.external.us_duration_demand.japan.us_duration.demand_schedule,adapter.external.japan,interface.us_duration.demand_schedule,market.us.treasury.secondary,interface.us_duration.demand_schedule,schedule,USD notional by instrument bucket,USD_notional,inbound_to_us,adapter.external.japan,profile-period interface contract,Expires with the provider binding or profile period.,NONE,typed market interface registration,fall back to adapter.external.external_world,No domestic clearing price or macro consequence is owned by the provider.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
tx.external.us_duration_demand.oil_exporters.foreign_reserves.flow,adapter.external.oil_exporters,interface.foreign_reserves.flow,market.us.treasury.secondary,interface.foreign_reserves.flow,quantity,USD notional per period,USD_notional_per_period,inbound_to_us,adapter.external.oil_exporters,profile-period interface contract,Expires with the provider binding or profile period.,NONE,typed market interface registration,fall back to adapter.external.external_world,No domestic clearing price or macro consequence is owned by the provider.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
tx.external.us_duration_demand.oil_exporters.us_duration.demand_schedule,adapter.external.oil_exporters,interface.us_duration.demand_schedule,market.us.treasury.secondary,interface.us_duration.demand_schedule,schedule,USD notional by instrument bucket,USD_notional,inbound_to_us,adapter.external.oil_exporters,profile-period interface contract,Expires with the provider binding or profile period.,NONE,typed market interface registration,fall back to adapter.external.external_world,No domestic clearing price or macro consequence is owned by the provider.,ECB 2006 annual report https://www.ecb.europa.eu/pub/pdf/annrep/ar2006en.pdf; BIS December 2006 cross-border banking https://www.bis.org/publ/qtrpdf/r_qt0612e.htm; U.S. Treasury TIC foreign holdings https://home.treasury.gov/news/press-releases/hp337; EIA petroleum imports https://www.eia.gov/totalenergy/data/annual/showtext.php?t=ptb0504; UNCTAD Review of Maritime Transport 2006 https://unctad.org/publication/review-maritime-transport-2006
tx.feed_boundary.to_hogs,adapter.input.amazon_basin.feed_grain,output.amazon_feed.availability,industry.amazon_basin.pork,input.amazon_hogs.feed_availability,availability,FEED_GRAIN mass price observation grade and delivery window,mass,boundary_to_industry,industry.amazon_basin.pork,contract/delivery delay,expires with offer,inventory.amazon_hogs.feed_grain,source-account offer delivery purchase inventory witness,South American feed adapter fallback,Exact delay and price UNKNOWN,06-design-discussion-representation-catalog.md:651
tx.green_coffee_flow.to_market,mechanism.flow.global.coffee,output.green_coffee_flow.supply_schedule,market.product.global.coffee,input.green_coffee_market.supply_schedule,distribution,GREEN_COFFEE quantity by price grade origin and window,mass,flow_to_market,market.product.global.coffee,submission delay,valid for clearing window,market eligibility,queue schedule-submission witness,market adapter consumes same interface,Exact market rules UNKNOWN,06-design-discussion-representation-catalog.md:665
tx.green_coffee_market.to_roasting,market.product.global.coffee,output.green_coffee_market.result,mechanism.transform.global.coffee_roasting,input.coffee_roasting.green_coffee_clearing,clearing_result,GREEN_COFFEE price allocation rationing failure title and settlement refs,mass,market_to_transformation,mechanism.transform.global.coffee_roasting,settlement/delivery delay,immutable result; allocation persists as titled inventory,state.coffee_roasting.capacity,clearing settlement title delivery recipe-queue witness,roasting adapter fallback,Exact buyer and settlement terms UNKNOWN,06-design-discussion-representation-catalog.md:674
tx.lumber_flow.to_market,mechanism.flow.global.timber,output.lumber_flow.supply_schedule,market.product.global.timber,input.lumber_market.supply_schedule,distribution,LUMBER quantity by price grade and window,standardized volume,flow_to_market,market.product.global.timber,submission delay,valid for clearing window,market eligibility,queue schedule-submission witness,market adapter consumes same interface,Exact market rules UNKNOWN,06-design-discussion-representation-catalog.md:664
tx.lumber_market.to_us_construction,market.product.global.timber,output.lumber_market.result,adapter.real.us.construction_inputs,input.us_construction.lumber_clearing,clearing_result,LUMBER price allocation rationing liquidity failure and settlement refs,standardized volume,market_to_basket,adapter.real.us.construction_inputs,publication/delivery delay,immutable result plus expiring opportunity,buyer budget and capacity,clearing allocation settlement scoped-delivery witness,market-adapter parity,Exact construction counterparties UNKNOWN,06-design-discussion-representation-catalog.md:673
tx.milling.to_lumber_flow,mechanism.transform.global.timber_milling,output.timber_milling.lumber_release,mechanism.flow.global.timber,input.lumber_flow.lumber_release,quantity,LUMBER by grade origin title and delivery window,standardized volume,transformation_to_flow,mechanism.flow.global.timber,dispatch delay,titled inventory persists,state.lumber_flow.capacity,recipe balanced-inventory release custody queue witness,flow adapter fallback,Exact delay UNKNOWN,06-design-discussion-representation-catalog.md:654
tx.pork_flow.to_market,mechanism.flow.global.pork,output.pork_flow.supply_schedule,market.product.global.pork,input.pork_market.supply_schedule,distribution,PORK quantity by price grade temperature and window,carcass-weight equivalent,flow_to_market,market.product.global.pork,submission delay,valid for clearing window,market eligibility,queue schedule-submission witness,market adapter consumes same interface,Exact market rules UNKNOWN,06-design-discussion-representation-catalog.md:666
tx.pork_market.to_us_food_baskets,market.product.global.pork,output.pork_market.result,adapter.real.us.food_baskets,input.us_food_baskets.pork_clearing,clearing_result,PORK price allocation rationing liquidity failure and settlement refs,carcass-weight equivalent,market_to_basket,adapter.real.us.food_baskets,publication/retail delay,immutable result plus perishable allocation,buyer budget and cold chain,clearing allocation settlement cold-chain scoped-delivery witness,market-adapter parity,Exact retail counterparties UNKNOWN,06-design-discussion-representation-catalog.md:677
tx.roasting.to_us_food_baskets,mechanism.transform.global.coffee_roasting,output.coffee_roasting.roasted_release,adapter.real.us.food_baskets,input.us_food_baskets.roasted_coffee,quantity,ROASTED_COFFEE mass unit cost quality title and window,mass,transformation_to_basket,adapter.real.us.food_baskets,wholesale/retail delay,inventory persists until sold consumed or lost,distribution capacity,recipe inventory title/custody delivery purchase witness,roasting adapter parity,Exact retail counterparties UNKNOWN,06-design-discussion-representation-catalog.md:676
tx.row_green_coffee.to_green_flow,adapter.supply.row.green_coffee,output.row_green_coffee.offer,mechanism.flow.global.coffee,input.green_coffee_flow.supply_offer,quantity,GREEN_COFFEE by grade origin crop year title contract and window,mass,row_to_flow,mechanism.flow.global.coffee,external delivery delay,offer expiry,state.green_coffee_flow.capacity,external source-account contract queue title/custody witness,adapter-provider parity,Exact source counterparties UNKNOWN,06-design-discussion-representation-catalog.md:660
tx.row_live_hogs.to_slaughter,adapter.supply.row.live_hogs,output.row_hogs.live_offer,mechanism.transform.global.hogs_slaughter,input.hogs_slaughter.live_offer,quantity,LIVE_HOGS with weight/health distribution origin title contract and window,headcount,row_to_transformation,mechanism.transform.global.hogs_slaughter,external transport delay,offer expiry,state.hogs_slaughter.capacity,external source-account contract transport inspection custody witness,adapter-provider parity,Exact source counterparties UNKNOWN,06-design-discussion-representation-catalog.md:662
tx.row_timber.to_milling,adapter.supply.row.timber,output.row_timber.offer,mechanism.transform.global.timber_milling,input.timber_milling.timber_offer,quantity,TIMBER by grade origin title contract and window,standardized solid cubic meter,row_to_transformation,mechanism.transform.global.timber_milling,external delivery delay,offer expiry,state.timber_milling.capacity,external source-account contract queue title/custody witness,adapter-provider parity,Exact source counterparties UNKNOWN,06-design-discussion-representation-catalog.md:658
tx.sa_coffee_residual.to_green_flow,industry.sa.coffee.aggregate,output.sa_coffee.green_offer,mechanism.flow.global.coffee,input.green_coffee_flow.supply_offer,quantity,GREEN_COFFEE by grade origin crop year title contract and window,mass,residual_to_flow,mechanism.flow.global.coffee,contract/delivery delay,offer expiry,state.green_coffee_flow.capacity,residual inventory contract queue title/custody witness,sector-mechanism fallback,Exact residual quantities UNKNOWN,06-design-discussion-representation-catalog.md:659
tx.sa_forestry_residual.to_milling,industry.sa.forestry.aggregate,output.sa_forestry.timber_offer,mechanism.transform.global.timber_milling,input.timber_milling.timber_offer,quantity,TIMBER by grade origin title contract and window,standardized solid cubic meter,residual_to_transformation,mechanism.transform.global.timber_milling,contract/delivery delay,offer expiry,state.timber_milling.capacity,residual inventory contract queue title/custody witness,sector-mechanism fallback,Exact residual quantities UNKNOWN,06-design-discussion-representation-catalog.md:657
tx.sa_hogs_residual.to_slaughter,industry.sa.pork.aggregate,output.sa_hogs.live_offer,mechanism.transform.global.hogs_slaughter,input.hogs_slaughter.live_offer,quantity,LIVE_HOGS with weight/health distribution origin title contract and window,headcount,residual_to_transformation,mechanism.transform.global.hogs_slaughter,contract/transport delay,offer expiry,state.hogs_slaughter.capacity,residual biological inventory contract transport inspection custody witness,sector-mechanism fallback,Exact residual quantities UNKNOWN,06-design-discussion-representation-catalog.md:661
tx.slaughter.to_pork_flow,mechanism.transform.global.hogs_slaughter,output.hogs_slaughter.pork_release,mechanism.flow.global.pork,input.pork_flow.pork_release,quantity,PORK by grade temperature origin title and window,carcass-weight equivalent,transformation_to_flow,mechanism.flow.global.pork,cold-chain dispatch delay,titled inventory persists until consumed/lost,state.pork_flow.capacity,recipe inspection balanced-inventory release custody queue witness,flow adapter fallback,Exact delay UNKNOWN,06-design-discussion-representation-catalog.md:663
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/closure/type_fidelity.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/closure/type_fidelity.csv
size_bytes: 8748
mode_octal: "0644"
modified_at_utc: 2026-09-04T04:01:28.032569Z
sha256: 56dcf9ced7e834025db4bde60f8ee64f18c2bf959130217ef5998e0f0495c5dc
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
type_id,fidelity_tier,is_default,provenance
type.adapter.resource_boundary,MECHANICAL_OR_ADAPTER,true,05-design-discussion-representation-bible.md:189
type.adapter.typed_boundary,MECHANICAL_OR_ADAPTER,true,06-design-discussion-representation-catalog.md:416
type.generator.regional_climate_hazard,MECHANICAL_OR_ADAPTER,true,06-design-discussion-representation-catalog.md:409
type.industry.regional_product,MECHANICAL_OR_ADAPTER,true,06-design-discussion-representation-catalog.md:411
type.industry.resource_production,MECHANICAL_OR_ADAPTER,true,05-design-discussion-representation-bible.md:172
type.institution.regional_bank,MECHANICAL_OR_ADAPTER,true,06-design-discussion-representation-catalog.md:900
type.instrument_family.deposits,MECHANICAL_OR_ADAPTER,true,06-design-discussion-representation-catalog.md:352;06-design-discussion-representation-catalog.md:1097;06-design-discussion-representation-catalog.md:1184;06-design-discussion-representation-catalog.md:1215
type.instrument_family.equity,MECHANICAL_OR_ADAPTER,true,06-design-discussion-representation-catalog.md:352;06-design-discussion-representation-catalog.md:1103;06-design-discussion-representation-catalog.md:1186;06-design-discussion-representation-catalog.md:1214
type.instrument_family.futures,MECHANICAL_OR_ADAPTER,true,06-design-discussion-representation-catalog.md:352;06-design-discussion-representation-catalog.md:1105;06-design-discussion-representation-catalog.md:1181;06-design-discussion-representation-catalog.md:1212
type.instrument_family.loans,MECHANICAL_OR_ADAPTER,true,06-design-discussion-representation-catalog.md:352;06-design-discussion-representation-catalog.md:1100;06-design-discussion-representation-catalog.md:1187;06-design-discussion-representation-catalog.md:1217
type.instrument_family.money_fund_share,MECHANICAL_OR_ADAPTER,true,06-design-discussion-representation-catalog.md:352;06-design-discussion-representation-catalog.md:1098;06-design-discussion-representation-catalog.md:1185;06-design-discussion-representation-catalog.md:1216
type.instrument_family.repo,MECHANICAL_OR_ADAPTER,true,06-design-discussion-representation-catalog.md:352;06-design-discussion-representation-catalog.md:1099;06-design-discussion-representation-catalog.md:1182;06-design-discussion-representation-catalog.md:1211
type.instrument_family.reserves,MECHANICAL_OR_ADAPTER,true,06-design-discussion-representation-catalog.md:352;06-design-discussion-representation-catalog.md:1096;06-design-discussion-representation-catalog.md:1183;06-design-discussion-representation-catalog.md:1213
type.instrument_family.tips,MECHANICAL_OR_ADAPTER,true,06-design-discussion-representation-catalog.md:352;06-design-discussion-representation-catalog.md:1095;06-design-discussion-representation-catalog.md:1218
type.instrument_family.treasury_bill,MECHANICAL_OR_ADAPTER,true,06-design-discussion-representation-catalog.md:352;06-design-discussion-representation-catalog.md:1092;06-design-discussion-representation-catalog.md:1208
type.instrument_family.treasury_bond,MECHANICAL_OR_ADAPTER,true,06-design-discussion-representation-catalog.md:352;06-design-discussion-representation-catalog.md:1094;06-design-discussion-representation-catalog.md:1180;06-design-discussion-representation-catalog.md:1210
type.instrument_family.treasury_note,MECHANICAL_OR_ADAPTER,true,06-design-discussion-representation-catalog.md:352;06-design-discussion-representation-catalog.md:1093;06-design-discussion-representation-catalog.md:1179;06-design-discussion-representation-catalog.md:1209
type.legal.resource_rights,MECHANICAL_OR_ADAPTER,true,05-design-discussion-representation-bible.md:180
type.market.product,MECHANICAL_OR_ADAPTER,true,06-design-discussion-representation-catalog.md:415
type.mechanical.bank.regional_aggregate,MECHANICAL_OR_ADAPTER,true,06-design-discussion-representation-catalog.md:900;06-design-discussion-representation-catalog.md:925-928
type.mechanical.product_demand,MECHANICAL_OR_ADAPTER,true,06-design-discussion-representation-catalog.md:413
type.mechanical.product_flow,MECHANICAL_OR_ADAPTER,true,06-design-discussion-representation-catalog.md:412
type.mechanical.regional_sector,MECHANICAL_OR_ADAPTER,true,06-design-discussion-representation-catalog.md:414
type.mechanical.resource_operation,MECHANICAL_OR_ADAPTER,true,05-design-discussion-representation-bible.md:177
type.organization_cohort.regional_bank,MECHANICAL_OR_ADAPTER,true,06-design-discussion-representation-catalog.md:900
type.process.ecological_stock,MECHANICAL_OR_ADAPTER,true,05-design-discussion-representation-bible.md:188
type.process.regional_climate_agriculture,MECHANICAL_OR_ADAPTER,true,06-design-discussion-representation-catalog.md:410
type.process.regional_resource_system,MECHANICAL_OR_ADAPTER,true,05-design-discussion-representation-bible.md:188;user-request:strategic-resource-regions
type.region.aggregate_scope,MECHANICAL_OR_ADAPTER,true,06-design-discussion-representation-catalog.md:408
type.region.ecological_scope,MECHANICAL_OR_ADAPTER,true,05-design-discussion-representation-bible.md:175
type.region.resource_macroregion,MECHANICAL_OR_ADAPTER,true,05-design-discussion-representation-bible.md:175;user-request:strategic-resource-regions
type.sovereign.jurisdiction_scope,MECHANICAL_OR_ADAPTER,true,05-design-discussion-representation-bible.md:174
type.person.default,NAMED_COGNITION,true,Representation Bible identity-clade contract.
type.office.default,MECHANICAL_OR_ADAPTER,true,Representation Bible identity-clade contract.
type.institution.default,MECHANICAL_OR_ADAPTER,true,Representation Bible identity-clade contract.
type.decision_body.default,MECHANICAL_OR_ADAPTER,true,Representation Bible identity-clade contract.
type.staff_unit.default,MECHANICAL_OR_ADAPTER,true,Representation Bible identity-clade contract.
type.person_population_cell.default,MECHANICAL_OR_ADAPTER,true,Representation Bible identity-clade contract.
type.household_cohort.default,MECHANICAL_OR_ADAPTER,true,Representation Bible identity-clade contract.
type.pop_lens.default,MECHANICAL_OR_ADAPTER,true,Representation Bible identity-clade contract.
type.organization_cohort.default,MECHANICAL_OR_ADAPTER,true,Representation Bible identity-clade contract.
type.named_firm.default,MECHANICAL_OR_ADAPTER,true,Representation Bible identity-clade contract.
type.industry_cohort.default,MECHANICAL_OR_ADAPTER,true,Representation Bible identity-clade contract.
type.coalition.default,MECHANICAL_OR_ADAPTER,true,Representation Bible identity-clade contract.
type.sovereign_system.default,MECHANICAL_OR_ADAPTER,true,Representation Bible identity-clade contract.
type.region.default,MECHANICAL_OR_ADAPTER,true,Representation Bible identity-clade contract.
type.market.default,MECHANICAL_OR_ADAPTER,true,Representation Bible identity-clade contract.
type.mechanical_system.default,MECHANICAL_OR_ADAPTER,true,Representation Bible identity-clade contract.
type.generator.default,MECHANICAL_OR_ADAPTER,true,Representation Bible identity-clade contract.
type.agreement.default,MECHANICAL_OR_ADAPTER,true,Representation Bible identity-clade contract.
type.legal_instrument.default,MECHANICAL_OR_ADAPTER,true,Representation Bible identity-clade contract.
type.federated_system.default,MECHANICAL_OR_ADAPTER,true,Representation Bible identity-clade contract.
type.facility.default,MECHANICAL_OR_ADAPTER,true,Representation Bible identity-clade contract.
type.published_reference.default,MECHANICAL_OR_ADAPTER,true,Representation Bible identity-clade contract.
type.outlet.default,MECHANICAL_OR_ADAPTER,true,Representation Bible identity-clade contract.
type.network.default,MECHANICAL_OR_ADAPTER,true,Representation Bible identity-clade contract.
type.scheduled_process.default,MECHANICAL_OR_ADAPTER,true,Representation Bible identity-clade contract.
type.record.default,MECHANICAL_OR_ADAPTER,true,Representation Bible identity-clade contract.
type.stateful_external_process.default,MECHANICAL_OR_ADAPTER,true,Representation Bible identity-clade contract.
type.boundary_adapter.default,MECHANICAL_OR_ADAPTER,true,Representation Bible identity-clade contract.
type.boundary_adapter.channel_provider,MECHANICAL_OR_ADAPTER,true,Channel-first external candidate atlas.
type.region.non_owning_scope,MECHANICAL_OR_ADAPTER,true,Channel-first external candidate atlas.
type.instrument_family.agency_mbs,MECHANICAL_OR_ADAPTER,true,Representation Catalog normative family inventory.
type.instrument_family.corporate_bonds,MECHANICAL_OR_ADAPTER,true,Representation Catalog normative family inventory.
type.instrument_family.swaps,MECHANICAL_OR_ADAPTER,true,Representation Catalog normative family inventory.
type.instrument_family.guarantees_credit_lines,MECHANICAL_OR_ADAPTER,true,Representation Catalog normative family inventory.
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/closure/types.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/closure/types.csv
size_bytes: 16244
mode_octal: "0644"
modified_at_utc: 2026-09-04T04:11:43.347949Z
sha256: 81c8936447febfeddd54a51d7037ff9fe824881bffc6e280a511cf92e8b3d5a5
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
type_id,display_name,identity_clade,cognition_class,required_fallback_description,provenance
type.adapter.resource_boundary,Resource-system boundary adapter,BoundaryAdapter,none,Endogenous provider or compatible boundary adapter,05-design-discussion-representation-bible.md:189
type.adapter.typed_boundary,Typed boundary adapter,BoundaryAdapter,none,Endogenous provider or adapter with same interface version,06-design-discussion-representation-catalog.md:416
type.agreement.default,Generic Agreement contract,Agreement,none,An incomplete non-selectable instance uses UNKNOWN; selectable instances require an instance-level lower-fidelity fallback.,Representation Bible identity-clade contract.
type.boundary_adapter.channel_provider,Channel-specific external provider,BoundaryAdapter,none,Segment providers terminate at the same channel residual; residual providers are terminal exogenous inputs.,Channel-first external candidate atlas.
type.boundary_adapter.default,Generic BoundaryAdapter contract,BoundaryAdapter,none,An incomplete non-selectable instance uses UNKNOWN; selectable instances require an instance-level lower-fidelity fallback.,Representation Bible identity-clade contract.
type.coalition.default,Generic Coalition contract,Coalition,none,An incomplete non-selectable instance uses UNKNOWN; selectable instances require an instance-level lower-fidelity fallback.,Representation Bible identity-clade contract.
type.decision_body.default,Generic DecisionBody contract,DecisionBody,none,An incomplete non-selectable instance uses UNKNOWN; selectable instances require an instance-level lower-fidelity fallback.,Representation Bible identity-clade contract.
type.facility.default,Generic Facility contract,Facility,none,An incomplete non-selectable instance uses UNKNOWN; selectable instances require an instance-level lower-fidelity fallback.,Representation Bible identity-clade contract.
type.federated_system.default,Generic FederatedSystem contract,FederatedSystem,none,An incomplete non-selectable instance uses UNKNOWN; selectable instances require an instance-level lower-fidelity fallback.,Representation Bible identity-clade contract.
type.generator.default,Generic Generator contract,Generator,none,An incomplete non-selectable instance uses UNKNOWN; selectable instances require an instance-level lower-fidelity fallback.,Representation Bible identity-clade contract.
type.generator.regional_climate_hazard,Regional climate hazard generator,Generator,none,Incident-tape BoundaryAdapter,06-design-discussion-representation-catalog.md:409
type.household_cohort.default,Generic HouseholdCohort contract,HouseholdCohort,none,An incomplete non-selectable instance uses UNKNOWN; selectable instances require an instance-level lower-fidelity fallback.,Representation Bible identity-clade contract.
type.industry.regional_product,Regional product industry,IndustryCohort,cohort_response,Product-flow or sector MechanicalSystem; compatible IndustryCohort retains residual reconciliation,06-design-discussion-representation-catalog.md:411
type.industry.resource_production,Resource-production industry cohort,IndustryCohort,cohort_response,Compatible sector mechanism or industry residual cohort,05-design-discussion-representation-bible.md:172
type.industry_cohort.default,Generic IndustryCohort contract,IndustryCohort,none,An incomplete non-selectable instance uses UNKNOWN; selectable instances require an instance-level lower-fidelity fallback.,Representation Bible identity-clade contract.
type.institution.default,Generic Institution contract,Institution,none,An incomplete non-selectable instance uses UNKNOWN; selectable instances require an instance-level lower-fidelity fallback.,Representation Bible identity-clade contract.
type.institution.regional_bank,Regional bank,Institution,participant_cognition,Compatible regional-bank organization cohort with exact named-plus-residual reconciliation.,06-design-discussion-representation-catalog.md:900
type.instrument_family.agency_mbs,Agency MBS,MechanicalSystem,none,Family vocabulary entry; instrument positions require instance-level representation.,Representation Catalog normative family inventory.
type.instrument_family.corporate_bonds,Corporate bonds,MechanicalSystem,none,Family vocabulary entry; instrument positions require instance-level representation.,Representation Catalog normative family inventory.
type.instrument_family.deposits,Deposits,MechanicalSystem,none,Type-level closed vocabulary; insurance is an account legal condition not a family split.,06-design-discussion-representation-catalog.md:352;06-design-discussion-representation-catalog.md:1097;06-design-discussion-representation-catalog.md:1184;06-design-discussion-representation-catalog.md:1215
type.instrument_family.equity,Equity,MechanicalSystem,none,Type-level closed vocabulary; tradable equity is distinct from institution accounting equity.,06-design-discussion-representation-catalog.md:352;06-design-discussion-representation-catalog.md:1103;06-design-discussion-representation-catalog.md:1186;06-design-discussion-representation-catalog.md:1214
type.instrument_family.futures,Futures,MechanicalSystem,none,Type-level closed vocabulary; specific contract/deliverable promotion requires pre-run extension.,06-design-discussion-representation-catalog.md:352;06-design-discussion-representation-catalog.md:1105;06-design-discussion-representation-catalog.md:1181;06-design-discussion-representation-catalog.md:1212
type.instrument_family.guarantees_credit_lines,Guarantees and credit lines,MechanicalSystem,none,Family vocabulary entry; instrument positions require instance-level representation.,Representation Catalog normative family inventory.
type.instrument_family.loans,Loans,MechanicalSystem,none,Type-level closed vocabulary; borrower-specific default requires pre-run promotion only when cohort state is insufficient.,06-design-discussion-representation-catalog.md:352;06-design-discussion-representation-catalog.md:1100;06-design-discussion-representation-catalog.md:1187;06-design-discussion-representation-catalog.md:1217
type.instrument_family.money_fund_share,Money-fund shares,MechanicalSystem,none,Type-level closed vocabulary; cash-lender interface may be a boundary adapter when not endogenous.,06-design-discussion-representation-catalog.md:352;06-design-discussion-representation-catalog.md:1098;06-design-discussion-representation-catalog.md:1185;06-design-discussion-representation-catalog.md:1216
type.instrument_family.repo,Repo,MechanicalSystem,none,Type-level closed vocabulary; individual bilateral terms and collateral ownership remain agreements/accounts.,06-design-discussion-representation-catalog.md:352;06-design-discussion-representation-catalog.md:1099;06-design-discussion-representation-catalog.md:1182;06-design-discussion-representation-catalog.md:1211
type.instrument_family.reserves,Reserves,MechanicalSystem,none,Type-level closed vocabulary; account owner and payment queue remain canonical.,06-design-discussion-representation-catalog.md:352;06-design-discussion-representation-catalog.md:1096;06-design-discussion-representation-catalog.md:1183;06-design-discussion-representation-catalog.md:1213
type.instrument_family.swaps,Swaps,MechanicalSystem,none,Family vocabulary entry; instrument positions require instance-level representation.,Representation Catalog normative family inventory.
type.instrument_family.tips,TIPS,MechanicalSystem,none,Type-level closed vocabulary with named inflation-reference dependency when activated.,06-design-discussion-representation-catalog.md:352;06-design-discussion-representation-catalog.md:1095;06-design-discussion-representation-catalog.md:1218
type.instrument_family.treasury_bill,Treasury bills,MechanicalSystem,none,Type-level closed vocabulary; positions remain owner-account entries.,06-design-discussion-representation-catalog.md:352;06-design-discussion-representation-catalog.md:1092;06-design-discussion-representation-catalog.md:1208
type.instrument_family.treasury_bond,Treasury bonds,MechanicalSystem,none,Type-level closed vocabulary; cash-bond bucket pairs with futures bucket for basis position.,06-design-discussion-representation-catalog.md:352;06-design-discussion-representation-catalog.md:1094;06-design-discussion-representation-catalog.md:1180;06-design-discussion-representation-catalog.md:1210
type.instrument_family.treasury_note,Treasury notes,MechanicalSystem,none,Type-level closed vocabulary; cash-note bucket pairs with futures bucket for basis position.,06-design-discussion-representation-catalog.md:352;06-design-discussion-representation-catalog.md:1093;06-design-discussion-representation-catalog.md:1179;06-design-discussion-representation-catalog.md:1209
type.legal.resource_rights,Resource-right legal instrument,LegalInstrument,none,Effective-dated legal boundary adapter,05-design-discussion-representation-bible.md:180
type.legal_instrument.default,Generic LegalInstrument contract,LegalInstrument,none,An incomplete non-selectable instance uses UNKNOWN; selectable instances require an instance-level lower-fidelity fallback.,Representation Bible identity-clade contract.
type.market.default,Generic Market contract,Market,none,An incomplete non-selectable instance uses UNKNOWN; selectable instances require an instance-level lower-fidelity fallback.,Representation Bible identity-clade contract.
type.market.product,Product market,Market,none,Market BoundaryAdapter with result parity,06-design-discussion-representation-catalog.md:415
type.mechanical.bank.regional_aggregate,Regional-bank aggregate boundary mechanism,MechanicalSystem,none,Fallback to a lower-fidelity bank-system boundary with account and transmission parity.,06-design-discussion-representation-catalog.md:900;06-design-discussion-representation-catalog.md:925-928
type.mechanical.product_demand,Product demand mechanism,MechanicalSystem,none,Demand BoundaryAdapter,06-design-discussion-representation-catalog.md:413
type.mechanical.product_flow,Product flow mechanism,MechanicalSystem,none,Interface-compatible BoundaryAdapter,06-design-discussion-representation-catalog.md:412
type.mechanical.regional_sector,Regional sector mechanism,MechanicalSystem,none,Compatible IndustryCohort or typed sector adapter,06-design-discussion-representation-catalog.md:414
type.mechanical.resource_operation,Resource extraction transformation or transport mechanism,MechanicalSystem,none,Interface-compatible boundary adapter,05-design-discussion-representation-bible.md:177
type.mechanical_system.default,Generic MechanicalSystem contract,MechanicalSystem,none,An incomplete non-selectable instance uses UNKNOWN; selectable instances require an instance-level lower-fidelity fallback.,Representation Bible identity-clade contract.
type.named_firm.default,Generic NamedFirm contract,NamedFirm,none,An incomplete non-selectable instance uses UNKNOWN; selectable instances require an instance-level lower-fidelity fallback.,Representation Bible identity-clade contract.
type.network.default,Generic Network contract,Network,none,An incomplete non-selectable instance uses UNKNOWN; selectable instances require an instance-level lower-fidelity fallback.,Representation Bible identity-clade contract.
type.office.default,Generic Office contract,Office,none,An incomplete non-selectable instance uses UNKNOWN; selectable instances require an instance-level lower-fidelity fallback.,Representation Bible identity-clade contract.
type.organization_cohort.default,Generic OrganizationCohort contract,OrganizationCohort,none,An incomplete non-selectable instance uses UNKNOWN; selectable instances require an instance-level lower-fidelity fallback.,Representation Bible identity-clade contract.
type.organization_cohort.regional_bank,Regional-bank organization cohort,OrganizationCohort,cohort_response,Sector mechanism preserving bank-sector boundary interfaces.,06-design-discussion-representation-catalog.md:900
type.outlet.default,Generic Outlet contract,Outlet,none,An incomplete non-selectable instance uses UNKNOWN; selectable instances require an instance-level lower-fidelity fallback.,Representation Bible identity-clade contract.
type.person.default,Generic Person contract,Person,named_cognition,An incomplete non-selectable instance uses UNKNOWN; selectable instances require an instance-level lower-fidelity fallback.,Representation Bible identity-clade contract.
type.person_population_cell.default,Generic PersonPopulationCell contract,PersonPopulationCell,none,An incomplete non-selectable instance uses UNKNOWN; selectable instances require an instance-level lower-fidelity fallback.,Representation Bible identity-clade contract.
type.pop_lens.default,Generic PopLens contract,PopLens,none,An incomplete non-selectable instance uses UNKNOWN; selectable instances require an instance-level lower-fidelity fallback.,Representation Bible identity-clade contract.
type.process.ecological_stock,Ecological stock and hydrology process,StatefulExternalProcess,none,Process boundary adapter with explicit external accounts,05-design-discussion-representation-bible.md:188
type.process.regional_climate_agriculture,Regional climate agriculture process,StatefulExternalProcess,none,Process BoundaryAdapter with output parity,06-design-discussion-representation-catalog.md:410
type.process.regional_resource_system,Regional resource conditions,StatefulExternalProcess,none,Process BoundaryAdapter preserving typed condition and resource outputs,05-design-discussion-representation-bible.md:188;user-request:strategic-resource-regions
type.published_reference.default,Generic PublishedReference contract,PublishedReference,none,An incomplete non-selectable instance uses UNKNOWN; selectable instances require an instance-level lower-fidelity fallback.,Representation Bible identity-clade contract.
type.record.default,Generic Record contract,Record,none,An incomplete non-selectable instance uses UNKNOWN; selectable instances require an instance-level lower-fidelity fallback.,Representation Bible identity-clade contract.
type.region.aggregate_scope,Aggregate region scope,Region,none,Broader Region aggregate or scenario external-region adapter,06-design-discussion-representation-catalog.md:408
type.region.default,Generic Region contract,Region,none,An incomplete non-selectable instance uses UNKNOWN; selectable instances require an instance-level lower-fidelity fallback.,Representation Bible identity-clade contract.
type.region.ecological_scope,Ecological region scope,Region,none,Broader Region or scenario boundary adapter,05-design-discussion-representation-bible.md:175
type.region.non_owning_scope,Non-owning external scope,Region,none,Scopes own no causal state and are not selected as providers.,Channel-first external candidate atlas.
type.region.resource_macroregion,Resource macroregion,Region,none,Broader aggregate Region or external-region BoundaryAdapter,05-design-discussion-representation-bible.md:175;user-request:strategic-resource-regions
type.scheduled_process.default,Generic ScheduledProcess contract,ScheduledProcess,none,An incomplete non-selectable instance uses UNKNOWN; selectable instances require an instance-level lower-fidelity fallback.,Representation Bible identity-clade contract.
type.sovereign.jurisdiction_scope,Sovereign jurisdiction composition root,SovereignSystem,none,External-region boundary adapter retaining declared outputs,05-design-discussion-representation-bible.md:174
type.sovereign_system.default,Generic SovereignSystem contract,SovereignSystem,none,An incomplete non-selectable instance uses UNKNOWN; selectable instances require an instance-level lower-fidelity fallback.,Representation Bible identity-clade contract.
type.staff_unit.default,Generic StaffUnit contract,StaffUnit,none,An incomplete non-selectable instance uses UNKNOWN; selectable instances require an instance-level lower-fidelity fallback.,Representation Bible identity-clade contract.
type.stateful_external_process.default,Generic StatefulExternalProcess contract,StatefulExternalProcess,none,An incomplete non-selectable instance uses UNKNOWN; selectable instances require an instance-level lower-fidelity fallback.,Representation Bible identity-clade contract.
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/closure/world_profiles.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/closure/world_profiles.csv
size_bytes: 434
mode_octal: "0644"
modified_at_utc: 2026-09-04T04:03:06.727063Z
sha256: 51863bb6ec6061278abbcd44adf932e6de2c9084111ba980f3b0876a9c82c80f
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
profile_id,display_name,effective_period,player_entry_id,planning_status,research_status,uncertainty_notes,provenance
profile.early_2006.bernankey,Early 2006 Bernankey campaign candidate,2006-02-01/2006-12-31,person.us.ben_bernankey,active_candidate,sketched,"Planning profile only; no representation manifest, replay hash, or runtime initialization is authored.",task.md campaign direction; Federal Reserve History chair chronology.
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/fed_treasury/action_domains.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/fed_treasury/action_domains.csv
size_bytes: 222
mode_octal: "0644"
modified_at_utc: 2026-09-04T04:05:37.626100Z
sha256: f50072d98f4f91a410fafe31bd0cdbed351da9eba72c3c238e7f0ec4d6d5d119
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
catalog_id,action_domain,provenance
body.us.federal_reserve.fomc,monetary_policy_directive,Federal Reserve Act and FOMC operating structure.
inst.us.treasury,treasury_debt_issuance,U.S. Treasury debt-management authority.
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/fed_treasury/composition_probe_instrument_buckets.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/fed_treasury/composition_probe_instrument_buckets.csv
size_bytes: 2203
mode_octal: "0644"
modified_at_utc: 2026-09-04T04:03:52.979868Z
sha256: cb7e9f20b606e38f7b68310d885b710cbaeb39250df3f785eb336df510f0104c
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
composition_probe_id,bucket_id,requirement,provenance
probe.composition.burrow_bank,bucket.deposits.first_slice,Exercise the family contract and active first-slice aggregation dimensions.,Representation Catalog composition-probe dependency cut.
probe.composition.burrow_bank,bucket.loans.first_slice,Exercise the family contract and active first-slice aggregation dimensions.,Representation Catalog composition-probe dependency cut.
probe.composition.burrow_bank,bucket.reserves.first_slice,Exercise the family contract and active first-slice aggregation dimensions.,Representation Catalog composition-probe dependency cut.
probe.composition.burrow_bank,bucket.equity.first_slice,Exercise the family contract and active first-slice aggregation dimensions.,Representation Catalog composition-probe dependency cut.
probe.composition.burrow_bank,bucket.repo.first_slice,Exercise the family contract and active first-slice aggregation dimensions.,Representation Catalog composition-probe dependency cut.
probe.composition.treasury_basis_trade,bucket.treasury_note.first_slice,Exercise the family contract and active first-slice aggregation dimensions.,Representation Catalog composition-probe dependency cut.
probe.composition.treasury_basis_trade,bucket.treasury_bond.first_slice,Exercise the family contract and active first-slice aggregation dimensions.,Representation Catalog composition-probe dependency cut.
probe.composition.treasury_basis_trade,bucket.repo.first_slice,Exercise the family contract and active first-slice aggregation dimensions.,Representation Catalog composition-probe dependency cut.
probe.composition.treasury_basis_trade,bucket.futures.first_slice,Exercise the family contract and active first-slice aggregation dimensions.,Representation Catalog composition-probe dependency cut.
probe.composition.treasury_basis_trade,bucket.reserves.first_slice,Exercise the family contract and active first-slice aggregation dimensions.,Representation Catalog composition-probe dependency cut.
probe.composition.treasury_basis_trade,bucket.equity.first_slice,Exercise the family contract and active first-slice aggregation dimensions.,Representation Catalog composition-probe dependency cut.
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/fed_treasury/entities.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/fed_treasury/entities.csv
size_bytes: 24754
mode_octal: "0644"
modified_at_utc: 2026-09-04T18:55:20.041825Z
sha256: 9e1f215c20063e59bffd6952f1d9e45e7277972d78efaac983bbcfcb2ecdd338
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
catalog_id,display_name,entry_class,instance_of,identity_clade,domain,selectable_in_manifest,definition_version,cognition_class,default_fidelity,research_status,completeness_state,fit,fallback_entry_id,residual_counterpart_id,uncertainty_notes,provenance
agreement.us.federal_reserve.swap_lines,Central Bank Swap-Line Agreements,instance,type.agreement.default,Agreement,Fed,false,1,none,MECHANICAL_OR_ADAPTER,named,identity_only,fits,UNKNOWN,NONE,Counterparties/terms UNKNOWN; each side operates a facility.,06-design-discussion-representation-catalog.md:339
agreement.us.repo.bilateral,Bilateral Repo Agreement,instance,type.agreement.default,Agreement,markets,false,2,none,MECHANICAL_OR_ADAPTER,researched,probe_complete,fits,UNKNOWN,NONE,"The MVP selects one explicit non-automatic-roll contract with cash, claim, maturity, haircut, and collateral-control terms.",12-structure-outline-bernankey-mvp-cycle.md:299-319
auth.us.primary_dealer_designation,Primary-Dealer Designation Regime,instance,type.legal_instrument.default,LegalInstrument,markets,false,1,none,MECHANICAL_OR_ADAPTER,named,identity_only,fits,UNKNOWN,NONE,Effective legal terms and designated members UNKNOWN.,06-design-discussion-representation-catalog.md:378
body.us.federal_reserve.board_voting,Board of Governors Voting Body,instance,type.decision_body.default,DecisionBody,Fed,false,1,participant_cognition,PARTICIPANT_DISTRIBUTION,named,typed,fits,UNKNOWN,NONE,Distinct from Board institution; member roster UNKNOWN.,06-design-discussion-representation-catalog.md:256
body.us.federal_reserve.fomc,Federal Open Market Committee,instance,type.decision_body.default,DecisionBody,Fed,false,1,participant_cognition,PARTICIPANT_DISTRIBUTION,researched,probe_complete,fits,UNKNOWN,NONE,The MVP initializes a bounded 2006 roster and voting procedure; broader membership remains outside the selected slice.,12-structure-outline-bernankey-mvp-cycle.md:195-285
body.us.federal_reserve.reserve_bank_boards,Reserve Bank Boards of Directors,instance,type.decision_body.default,DecisionBody,Fed,false,1,participant_cognition,PARTICIPANT_DISTRIBUTION,named,identity_only,fits,UNKNOWN,NONE,Individual boards and members UNKNOWN; discount rate subject to Board review.,06-design-discussion-representation-catalog.md:257
body.us.treasury.tbac,Treasury Borrowing Advisory Committee,instance,type.decision_body.default,DecisionBody,Treasury,false,1,participant_cognition,PARTICIPANT_DISTRIBUTION,named,typed,fits,UNKNOWN,NONE,Advisory only; recommendations are claims not authorizations.,06-design-discussion-representation-catalog.md:259
facility.us.federal_reserve.discount_window,Discount Window,instance,type.facility.default,Facility,Fed,false,1,none,MECHANICAL_OR_ADAPTER,named,typed,fits,UNKNOWN,NONE,Operating Reserve Bank depends on borrower; terms UNKNOWN.,06-design-discussion-representation-catalog.md:338;07-design-discussion-burrow-composition-probe.md:143
facility.us.federal_reserve.fima_repo,FIMA Repo Facility,instance,type.facility.default,Facility,Fed,false,1,none,MECHANICAL_OR_ADAPTER,named,identity_only,fits,UNKNOWN,NONE,Terms/eligible account holders UNKNOWN.,06-design-discussion-representation-catalog.md:338;06-design-discussion-representation-catalog.md:378
facility.us.federal_reserve.rrp,Reverse Repo Facility,instance,type.facility.default,Facility,Fed,false,1,none,MECHANICAL_OR_ADAPTER,named,identity_only,fits,UNKNOWN,NONE,Terms/eligibility UNKNOWN.,06-design-discussion-representation-catalog.md:338
facility.us.federal_reserve.section_13_3,Section 13(3) Facility,instance,type.facility.default,Facility,Fed,false,1,none,MECHANICAL_OR_ADAPTER,named,identity_only,fits,UNKNOWN,NONE,Specific vehicle and selected era UNKNOWN.,06-design-discussion-representation-catalog.md:338;07-design-discussion-burrow-composition-probe.md:160-173
facility.us.federal_reserve.srf,Standing Repo Facility,instance,type.facility.default,Facility,Fed,false,1,none,MECHANICAL_OR_ADAPTER,named,typed,fits,UNKNOWN,NONE,Terms/caps/eligibility and period UNKNOWN; FOMC authorizes terms as stated.,06-design-discussion-representation-catalog.md:997-1020
federation.us.federal_reserve,Federal Reserve System,instance,type.federated_system.default,FederatedSystem,Fed,false,1,none,MECHANICAL_OR_ADAPTER,named,structural,fits,UNKNOWN,NONE,Board/Reserve Bank roster has a stated twelve-bank composition; current period UNKNOWN; consolidated balance sheet derived and system owns no stocks.,06-design-discussion-representation-catalog.md:269;06-design-discussion-representation-catalog.md:955-960
inst.us.cme,CME,instance,type.institution.default,Institution,markets,false,1,participant_cognition,PARTICIPANT_DISTRIBUTION,named,identity_only,fits,UNKNOWN,NONE,Promoted clearing subobject and exact organization details UNKNOWN.,06-design-discussion-representation-catalog.md:282;06-design-discussion-representation-catalog.md:345
inst.us.federal_reserve.board,Board of Governors,instance,type.institution.default,Institution,Fed,false,1,participant_cognition,PARTICIPANT_DISTRIBUTION,researched,probe_complete,fits,UNKNOWN,NONE,The MVP retains Board governance and authority identity without assigning SOMA assets to the Board.,12-structure-outline-bernankey-mvp-cycle.md:195-285
inst.us.federal_reserve.new_york,Federal Reserve Bank of New York,instance,type.institution.default,Institution,Fed,false,1,participant_cognition,PARTICIPANT_DISTRIBUTION,researched,probe_complete,fits,UNKNOWN,NONE,The MVP records directive-bounded Desk execution while settlement and market ownership remain separate.,12-structure-outline-bernankey-mvp-cycle.md:195-285
inst.us.federal_reserve.reserve_banks_other,Other Eleven Federal Reserve Banks,instance,type.institution.default,Institution,Fed,false,1,participant_cognition,ORGANIZATION_COHORT_RESPONSE,named,identity_only,fits,UNKNOWN,NONE,Individual identities intentionally shallow; separately chartered corporations with member-bank capital.,06-design-discussion-representation-catalog.md:271;06-design-discussion-representation-catalog.md:960
inst.us.ficc,FICC,instance,type.institution.default,Institution,markets,false,1,participant_cognition,PARTICIPANT_DISTRIBUTION,named,typed,fits,UNKNOWN,NONE,Owns discretionary margin/membership/default-management functions.,06-design-discussion-representation-catalog.md:282;06-design-discussion-representation-catalog.md:337
inst.us.leveraged_funds,Major Leveraged Funds,instance,type.organization_cohort.default,OrganizationCohort,markets,false,2,cohort_response,ORGANIZATION_COHORT_RESPONSE,researched,probe_complete,fits,UNKNOWN,NONE,"The MVP cohort owns cash, Treasury inventory, repo obligation, collateral encumbrance, leverage limits, and liquidity-driven order behavior.",12-structure-outline-bernankey-mvp-cycle.md:299-319
inst.us.primary_dealers,Primary Dealers,instance,type.organization_cohort.default,OrganizationCohort,markets,false,1,cohort_response,ORGANIZATION_COHORT_RESPONSE,named,typed,fits,UNKNOWN,NONE,Membership is a designation/eligibility affiliation; firms UNKNOWN.,06-design-discussion-representation-catalog.md:278;06-design-discussion-representation-catalog.md:378
inst.us.systemically_important_banks,Systemically Important Banks,instance,type.organization_cohort.default,OrganizationCohort,markets,false,1,cohort_response,ORGANIZATION_COHORT_RESPONSE,named,identity_only,fits,UNKNOWN,NONE,Individual banks UNKNOWN.,06-design-discussion-representation-catalog.md:278;04-design-discussion-minimum-simulation-kernel.md:808
inst.us.treasury,U.S. Treasury,instance,type.institution.default,Institution,Treasury,false,1,participant_cognition,PARTICIPANT_DISTRIBUTION,named,typed,fits,UNKNOWN,NONE,Includes ESF as distinct account with distinct authority.,06-design-discussion-representation-catalog.md:272
legal.us.federal_reserve.section_13_3,Section 13(3) Legal Regime,instance,type.legal_instrument.default,LegalInstrument,Fed,false,1,none,MECHANICAL_OR_ADAPTER,named,typed,fits,UNKNOWN,NONE,Effective clause varies by selected period; Treasury-consent condition not asserted absent era.,06-design-discussion-representation-catalog.md:1519-1521;07-design-discussion-burrow-composition-probe.md:160-173
market.us.treasury.futures,Treasury Futures Market,instance,type.market.default,Market,markets,false,1,none,MECHANICAL_OR_ADAPTER,named,typed,fits,UNKNOWN,NONE,Specific contracts/deliverable baskets UNKNOWN.,06-design-discussion-representation-catalog.md:345;06-design-discussion-representation-catalog.md:1181
market.us.treasury.secondary,Treasury Secondary Market by Maturity Bucket,instance,type.market.default,Market,markets,false,2,none,MECHANICAL_OR_ADAPTER,researched,probe_complete,fits,UNKNOWN,NONE,"The MVP selects a 5-10 year maturity bucket with bounded price formation, allocation, rationing, and explicit clearing failure.",12-structure-outline-bernankey-mvp-cycle.md:299-319
mechanism.us.chips,CHIPS,instance,type.mechanical_system.default,MechanicalSystem,markets,false,1,none,MECHANICAL_OR_ADAPTER,named,identity_only,fits,UNKNOWN,NONE,Operator and operating-window parameters UNKNOWN.,06-design-discussion-representation-catalog.md:342
mechanism.us.fedwire.funds,Fedwire Funds,instance,type.mechanical_system.default,MechanicalSystem,Fed,false,1,none,MECHANICAL_OR_ADAPTER,named,identity_only,fits,UNKNOWN,NONE,Operator and operating-window parameters UNKNOWN.,06-design-discussion-representation-catalog.md:342
mechanism.us.fedwire.securities,Fedwire Securities,instance,type.mechanical_system.default,MechanicalSystem,Fed,false,1,none,MECHANICAL_OR_ADAPTER,named,identity_only,fits,UNKNOWN,NONE,Operator and operating-window parameters UNKNOWN.,06-design-discussion-representation-catalog.md:342
mechanism.us.ficc.sponsored_gcf_repo,FICC Sponsored and GCF Repo Clearing Subobject,instance,type.mechanical_system.default,MechanicalSystem,markets,false,1,none,MECHANICAL_OR_ADAPTER,named,typed,fits,UNKNOWN,NONE,Promoted clearing subobject; matching/netting engine; FICC parent owner.,06-design-discussion-representation-catalog.md:337
mechanism.us.nss,National Settlement Service,instance,type.mechanical_system.default,MechanicalSystem,Fed,false,1,none,MECHANICAL_OR_ADAPTER,named,identity_only,fits,UNKNOWN,NONE,Operator and operating-window parameters UNKNOWN.,06-design-discussion-representation-catalog.md:342
office.us.federal_reserve.board_chair,Board Chair,instance,type.office.default,Office,Fed,false,1,none,LIMITED_ROLE_HOLDER,researched,probe_complete,fits,UNKNOWN,NONE,The 2006 office tenure is initialized separately from FOMC collective authority.,12-structure-outline-bernankey-mvp-cycle.md:195-285
office.us.federal_reserve.chief_of_staff,Chair's Chief of Staff,instance,type.office.default,Office,Fed,false,1,none,LIMITED_ROLE_HOLDER,sketched,probe_complete,fits,UNKNOWN,NONE,The MVP initializes a limited role-holder and agenda access without granting policy authority.,12-structure-outline-bernankey-mvp-cycle.md:195-285
office.us.federal_reserve.division_director,Division Director,instance,type.office.default,Office,Fed,false,1,none,LIMITED_ROLE_HOLDER,named,identity_only,fits,UNKNOWN,NONE,Named divisions held as staff units; individual roles/holders UNKNOWN.,06-design-discussion-representation-catalog.md:246
office.us.federal_reserve.fomc_chair,FOMC Chair,instance,type.office.default,Office,Fed,false,1,none,LIMITED_ROLE_HOLDER,researched,probe_complete,fits,UNKNOWN,NONE,The Chair may place a package before the FOMC but cannot direct the Desk without a certified Committee decision.,12-structure-outline-bernankey-mvp-cycle.md:195-285
office.us.federal_reserve.governor,Governor,instance,type.office.default,Office,Fed,false,1,none,LIMITED_ROLE_HOLDER,sketched,probe_complete,fits,UNKNOWN,NONE,The MVP uses two bounded role-holders with private sourced beliefs and separate votes.,12-structure-outline-bernankey-mvp-cycle.md:195-285
office.us.federal_reserve.ny_president,Federal Reserve Bank of New York President,instance,type.office.default,Office,Fed,false,1,none,LIMITED_ROLE_HOLDER,named,identity_only,fits,UNKNOWN,NONE,Current holder UNKNOWN; FOMC vice-chair status is custom not statute.,06-design-discussion-representation-catalog.md:244
office.us.federal_reserve.reserve_bank_president,Reserve Bank President,instance,type.office.default,Office,Fed,false,1,none,LIMITED_ROLE_HOLDER,named,identity_only,fits,UNKNOWN,NONE,Individual presidents UNKNOWN; NY FOMC vice-chair custom and voting rotation statutory.,06-design-discussion-representation-catalog.md:244
office.us.federal_reserve.vice_chair,Vice Chair,instance,type.office.default,Office,Fed,false,1,none,LIMITED_ROLE_HOLDER,named,identity_only,fits,UNKNOWN,NONE,Holder UNKNOWN.,06-design-discussion-representation-catalog.md:243
office.us.federal_reserve.vice_chair_supervision,Vice Chair for Supervision,instance,type.office.default,Office,Fed,false,1,none,LIMITED_ROLE_HOLDER,named,identity_only,fits,UNKNOWN,NONE,Statutory powers distinguished; holder UNKNOWN.,06-design-discussion-representation-catalog.md:243
office.us.treasury.secretary,Treasury Secretary Office,instance,type.office.default,Office,Treasury,false,1,none,LIMITED_ROLE_HOLDER,named,identity_only,fits,UNKNOWN,NONE,Office-holder UNKNOWN.,06-design-discussion-representation-catalog.md:247
person.us.alan_greenspaniel,Alan Greenspaniel,instance,type.person.default,Person,fed_treasury,false,1,named_cognition,NAMED_COGNITION,typed,typed,fits,UNKNOWN,NONE,No later-campaign state or policy model is authored.,Federal Reserve History: https://www.federalreservehistory.org/people/federal-reserve-chair
person.us.ben_bernankey,Ben Bernankey,instance,type.person.default,Person,fed_treasury,false,1,named_cognition,NAMED_COGNITION,typed,probe_complete,fits,UNKNOWN,NONE,Phase 1 freezes identity and an empty private cognition ledger; no policy model is authored.,12-structure-outline-bernankey-mvp-cycle.md:174-177; Federal Reserve History: https://www.federalreservehistory.org/people/federal-reserve-chair
person.us.janet_jackrabbit,Janet Jackrabbit,instance,type.person.default,Person,fed_treasury,false,1,named_cognition,NAMED_COGNITION,typed,typed,fits,UNKNOWN,NONE,No later-campaign state or policy model is authored.,Federal Reserve History: https://www.federalreservehistory.org/people/federal-reserve-chair
person.us.jerome_owl,Jerome Owl,instance,type.person.default,Person,fed_treasury,false,1,named_cognition,NAMED_COGNITION,typed,typed,fits,UNKNOWN,NONE,No later-campaign state or policy model is authored.,Federal Reserve History: https://www.federalreservehistory.org/people/federal-reserve-chair
person.us.kevin_boarsh,Kevin Boarsh,instance,type.person.default,Person,fed_treasury,false,1,named_cognition,NAMED_COGNITION,typed,typed,fits,UNKNOWN,NONE,No later-campaign state or policy model is authored.,Federal Reserve History: https://www.federalreservehistory.org/people/federal-reserve-chair
person.us.paul_vulture,Paul Vulture,instance,type.person.default,Person,fed_treasury,false,1,named_cognition,NAMED_COGNITION,typed,typed,fits,UNKNOWN,NONE,No later-campaign state or policy model is authored.,Federal Reserve History: https://www.federalreservehistory.org/people/federal-reserve-chair
person.us.primary_dealer.rates_head,Primary-dealer Rates Head,instance,type.person.default,Person,markets,false,1,named_cognition,LIMITED_ROLE_HOLDER,named,identity_only,fits,UNKNOWN,NONE,Individual firms and people UNKNOWN; promoted only where discretion changes channel.,06-design-discussion-representation-catalog.md:250
person.us.treasury.secretary,Treasury Secretary,instance,type.person.default,Person,Treasury,false,1,named_cognition,NAMED_COGNITION,named,identity_only,fits,UNKNOWN,NONE,Current person and period UNKNOWN.,task.md:128;06-design-discussion-representation-catalog.md:247
record.us.federal_reserve.analytical_task,Bounded Analytical Task,instance,type.record.default,Record,Fed,false,1,none,MECHANICAL_OR_ADAPTER,sketched,probe_complete,fits,UNKNOWN,NONE,Only the selected dealer-capacity comparison and its assignment deadline displacement and result are represented.,12-structure-outline-bernankey-mvp-cycle.md:388-457
record.us.federal_reserve.assessment,Staff Assessment,instance,type.record.default,Record,Fed,false,1,none,MECHANICAL_OR_ADAPTER,sketched,probe_complete,fits,UNKNOWN,NONE,Only the Markets follow-up with sourced uncertainty alternative assessments and Monetary Affairs dissent is represented.,12-structure-outline-bernankey-mvp-cycle.md:388-457
record.us.federal_reserve.case_file,Case File,instance,type.record.default,Record,Fed,false,1,none,MECHANICAL_OR_ADAPTER,named,identity_only,fits,UNKNOWN,NONE,Specific case/custody UNKNOWN.,06-design-discussion-representation-catalog.md:379;07-design-discussion-burrow-composition-probe.md:187
record.us.federal_reserve.chairmanship_program,Chairmanship Program,instance,type.record.default,Record,Fed,false,1,none,MECHANICAL_OR_ADAPTER,named,identity_only,fits,UNKNOWN,NONE,Specific program UNKNOWN.,06-design-discussion-representation-catalog.md:379
record.us.federal_reserve.discount_rate,Discount Rate Authorized Term,instance,type.record.default,Record,Fed,false,1,none,MECHANICAL_OR_ADAPTER,named,identity_only,strained,UNKNOWN,NONE,Classified as a facility term or authorized institution state; exact rate UNKNOWN.,06-design-discussion-representation-catalog.md:341
record.us.federal_reserve.institutional_project,Institutional Project,instance,type.record.default,Record,Fed,false,1,none,MECHANICAL_OR_ADAPTER,named,identity_only,fits,UNKNOWN,NONE,Specific project UNKNOWN.,06-design-discussion-representation-catalog.md:379
record.us.federal_reserve.iorb,Interest on Reserve Balances Authorized Term,instance,type.record.default,Record,Fed,false,1,none,MECHANICAL_OR_ADAPTER,named,identity_only,strained,UNKNOWN,NONE,Classified as a facility term or authorized institution state; not a market/mechanism.,06-design-discussion-representation-catalog.md:341
record.us.federal_reserve.legacy_dossier,Legacy Dossier,instance,type.record.default,Record,Fed,false,1,none,MECHANICAL_OR_ADAPTER,named,identity_only,fits,UNKNOWN,NONE,Specific dossier/custody UNKNOWN.,06-design-discussion-representation-catalog.md:379
record.us.federal_reserve.policy_package,Policy Package,instance,type.record.default,Record,Fed,false,1,none,MECHANICAL_OR_ADAPTER,sketched,probe_complete,fits,UNKNOWN,NONE,The MVP records three prepared packages and any Committee narrowing without assigning market outcomes.,12-structure-outline-bernankey-mvp-cycle.md:233-245
record.us.federal_reserve.target_range,Policy Target Range Authorized Objective,instance,type.record.default,Record,Fed,false,1,none,MECHANICAL_OR_ADAPTER,named,identity_only,strained,UNKNOWN,NONE,Authorized objective that desk implements toward; exact target UNKNOWN.,06-design-discussion-representation-catalog.md:341
record.us.treasury.esf,Exchange Stabilization Fund Account,instance,type.record.default,Record,Treasury,false,1,none,MECHANICAL_OR_ADAPTER,named,identity_only,fits,UNKNOWN,NONE,Described as a distinct account with distinct authority; exact legal/account structure UNKNOWN.,06-design-discussion-representation-catalog.md:272
record.us.treasury.indemnification_13_3,Treasury ESF Indemnification of 13(3) Facility,instance,type.record.default,Record,Treasury,false,1,none,MECHANICAL_OR_ADAPTER,named,identity_only,strained,UNKNOWN,NONE,Requires agreement authority and coalition; selected facility/era and terms UNKNOWN.,06-design-discussion-representation-catalog.md:363
reference.us.federal_reserve.effr,Effective Federal Funds Rate,instance,type.published_reference.default,PublishedReference,Fed,false,1,none,MECHANICAL_OR_ADAPTER,named,identity_only,fits,UNKNOWN,NONE,Publisher/method UNKNOWN.,06-design-discussion-representation-catalog.md:353
reference.us.federal_reserve.obfr,Overnight Bank Funding Rate,instance,type.published_reference.default,PublishedReference,Fed,false,1,none,MECHANICAL_OR_ADAPTER,named,identity_only,fits,UNKNOWN,NONE,Publisher/method UNKNOWN.,06-design-discussion-representation-catalog.md:353
schedule.us.federal_reserve.blackout,FOMC Blackout Period,instance,type.scheduled_process.default,ScheduledProcess,Fed,false,1,none,MECHANICAL_OR_ADAPTER,researched,probe_complete,fits,UNKNOWN,NONE,The MVP derives the communication gate from the selected published FOMC occurrence.,12-structure-outline-bernankey-mvp-cycle.md:278-285
schedule.us.federal_reserve.fomc,FOMC Calendar,instance,type.scheduled_process.default,ScheduledProcess,Fed,false,1,none,MECHANICAL_OR_ADAPTER,researched,probe_complete,fits,UNKNOWN,NONE,The March 2006 occurrence and its derived blackout window are initialized for the bounded cycle.,12-structure-outline-bernankey-mvp-cycle.md:278-285
schedule.us.treasury.auction,Treasury Auction Calendar,instance,type.scheduled_process.default,ScheduledProcess,Treasury,false,1,none,MECHANICAL_OR_ADAPTER,named,typed,fits,UNKNOWN,NONE,Published occurrences/parameters UNKNOWN.,06-design-discussion-representation-catalog.md:377
staff.us.federal_reserve.communications,Division of Communications,instance,type.staff_unit.default,StaffUnit,Fed,false,1,participant_cognition,PARTICIPANT_DISTRIBUTION,sketched,probe_complete,fits,UNKNOWN,NONE,The MVP retains bounded access methods and one capacity unit; publication work remains Phase 5.,12-structure-outline-bernankey-mvp-cycle.md:397-415
staff.us.federal_reserve.financial_stability,Division of Financial Stability,instance,type.staff_unit.default,StaffUnit,Fed,false,1,participant_cognition,PARTICIPANT_DISTRIBUTION,named,identity_only,fits,UNKNOWN,NONE,Director UNKNOWN.,06-design-discussion-representation-catalog.md:246
staff.us.federal_reserve.international,Division of International,instance,type.staff_unit.default,StaffUnit,Fed,false,1,participant_cognition,PARTICIPANT_DISTRIBUTION,named,identity_only,fits,UNKNOWN,NONE,Director UNKNOWN.,06-design-discussion-representation-catalog.md:246
staff.us.federal_reserve.legal,Division of Legal,instance,type.staff_unit.default,StaffUnit,Fed,false,1,participant_cognition,PARTICIPANT_DISTRIBUTION,named,identity_only,fits,UNKNOWN,NONE,Director UNKNOWN.,06-design-discussion-representation-catalog.md:246
staff.us.federal_reserve.markets,Division of Markets,instance,type.staff_unit.default,StaffUnit,Fed,false,1,participant_cognition,PARTICIPANT_DISTRIBUTION,sketched,probe_complete,fits,UNKNOWN,NONE,Owns the selected request capacity confidential evidence assessment and foreign-demand displacement.,12-structure-outline-bernankey-mvp-cycle.md:388-457
staff.us.federal_reserve.monetary_affairs,Division of Monetary Affairs,instance,type.staff_unit.default,StaffUnit,Fed,false,1,participant_cognition,PARTICIPANT_DISTRIBUTION,sketched,probe_complete,fits,UNKNOWN,NONE,The MVP retains bounded policy methods and authors the explicit dissent inside the selected assessment.,12-structure-outline-bernankey-mvp-cycle.md:388-457
staff.us.federal_reserve.ny_markets_group,New York Fed Markets Group,instance,type.staff_unit.default,StaffUnit,Fed,false,1,participant_cognition,PARTICIPANT_DISTRIBUTION,named,typed,fits,UNKNOWN,NONE,Owns SOMA execution; exact desk topology UNKNOWN.,06-design-discussion-representation-catalog.md:261;07-design-discussion-burrow-composition-probe.md:144
staff.us.federal_reserve.research,Division of Research,instance,type.staff_unit.default,StaffUnit,Fed,false,1,participant_cognition,PARTICIPANT_DISTRIBUTION,named,identity_only,fits,UNKNOWN,NONE,Director UNKNOWN.,06-design-discussion-representation-catalog.md:246
staff.us.federal_reserve.supervision,Division of Supervision,instance,type.staff_unit.default,StaffUnit,Fed,false,1,participant_cognition,PARTICIPANT_DISTRIBUTION,named,identity_only,fits,UNKNOWN,NONE,Director UNKNOWN.,06-design-discussion-representation-catalog.md:246
staff.us.treasury.debt_management,Office of Debt Management,instance,type.staff_unit.default,StaffUnit,Treasury,false,1,participant_cognition,PARTICIPANT_DISTRIBUTION,named,identity_only,fits,UNKNOWN,NONE,Exact office topology UNKNOWN.,06-design-discussion-representation-catalog.md:262
staff.us.treasury.ofr,Office of Financial Research,instance,type.staff_unit.default,StaffUnit,Treasury,false,1,participant_cognition,PARTICIPANT_DISTRIBUTION,named,identity_only,fits,UNKNOWN,NONE,Exact remit and topology UNKNOWN.,06-design-discussion-representation-catalog.md:262
stateful.us.treasury.duration_supply,Treasury Duration Supply Process,instance,type.stateful_external_process.default,StatefulExternalProcess,Treasury,false,1,none,MECHANICAL_OR_ADAPTER,sketched,identity_only,fits,UNKNOWN,NONE,Mechanism ownership/solver provisional; supplies maturity buckets.,04-design-discussion-minimum-simulation-kernel.md:173-184;04-design-discussion-minimum-simulation-kernel.md:717
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/fed_treasury/entity_authority_sources.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/fed_treasury/entity_authority_sources.csv
size_bytes: 243
mode_octal: "0644"
modified_at_utc: 2026-09-04T04:04:58.401260Z
sha256: 59083eff44a82a796bb8e9a1c08d19c8d2601a0c30d140617fdcc60c44a522a2
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
catalog_id,authority_source_id,authority_source_kind,provenance
person.us.ben_bernankey,office.us.federal_reserve.board_chair,effective_office_holding,Federal Reserve History: https://www.federalreservehistory.org/people/federal-reserve-chair
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/fed_treasury/instrument_bucket_dimensions.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/fed_treasury/instrument_bucket_dimensions.csv
size_bytes: 7446
mode_octal: "0644"
modified_at_utc: 2026-09-04T04:02:02.538851Z
sha256: 67a95d3d4c2a26212ea3aba682eec4cb52026f26fea9a2b74eb8c24c8279a020
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
bucket_id,bucket_dimension,bucket_value,calibration_requirement,provenance
bucket.treasury_bill.first_slice,remaining_maturity_band,short,Initializer must provide effective-period numeric cutoffs.,Representation Catalog lines 1176-1195.
bucket.treasury_bill.first_slice,rate_exposure,short_rate_reinvestment,NONE,Representation Catalog lines 1176-1195.
bucket.treasury_bill.first_slice,liquidity_cohort,benchmark_or_seasoned,NONE,Representation Catalog lines 1176-1195.
bucket.treasury_bill.first_slice,repo_collateral_class,general_collateral,NONE,Representation Catalog lines 1176-1195.
bucket.treasury_bill.first_slice,currency,USD,NONE,Representation Catalog lines 1176-1195.
bucket.treasury_note.first_slice,remaining_maturity_band,short|intermediate|long,Initializer must provide effective-period numeric cutoffs.,Representation Catalog lines 1176-1195.
bucket.treasury_note.first_slice,key_rate_duration,note_curve,NONE,Representation Catalog lines 1176-1195.
bucket.treasury_note.first_slice,liquidity_cohort,on_the_run|off_the_run,NONE,Representation Catalog lines 1176-1195.
bucket.treasury_note.first_slice,basis_role,cash_note_futures_basis,NONE,Representation Catalog lines 1176-1195.
bucket.treasury_note.first_slice,repo_collateral_class,treasury_note,NONE,Representation Catalog lines 1176-1195.
bucket.treasury_note.first_slice,currency,USD,NONE,Representation Catalog lines 1176-1195.
bucket.treasury_bond.first_slice,remaining_maturity_band,long,Initializer must provide effective-period numeric cutoffs.,Representation Catalog lines 1176-1195.
bucket.treasury_bond.first_slice,key_rate_duration,long_curve,NONE,Representation Catalog lines 1176-1195.
bucket.treasury_bond.first_slice,liquidity_cohort,on_the_run|off_the_run,NONE,Representation Catalog lines 1176-1195.
bucket.treasury_bond.first_slice,basis_role,cash_bond_futures_basis,NONE,Representation Catalog lines 1176-1195.
bucket.treasury_bond.first_slice,repo_collateral_class,treasury_bond,NONE,Representation Catalog lines 1176-1195.
bucket.treasury_bond.first_slice,currency,USD,NONE,Representation Catalog lines 1176-1195.
bucket.reserves.first_slice,settlement_term,current_balance,NONE,Representation Catalog lines 1176-1195.
bucket.reserves.first_slice,currency,USD,NONE,Representation Catalog lines 1176-1195.
bucket.reserves.first_slice,settlement_system,federal_reserve,NONE,Representation Catalog lines 1176-1195.
bucket.reserves.first_slice,account_eligibility,eligible_holder,NONE,Representation Catalog lines 1176-1195.
bucket.reserves.first_slice,remuneration_class,administered,NONE,Representation Catalog lines 1176-1195.
bucket.reserves.first_slice,balance_availability,available|reserved,NONE,Representation Catalog lines 1176-1195.
bucket.deposits.first_slice,account_term,demand|notice|term,NONE,Representation Catalog lines 1176-1195.
bucket.deposits.first_slice,repricing_band,fixed|floating|administered,Initializer must provide effective-period numeric cutoffs.,Representation Catalog lines 1176-1195.
bucket.deposits.first_slice,currency,USD,NONE,Representation Catalog lines 1176-1195.
bucket.deposits.first_slice,issuer_class,bank_or_cohort,NONE,Representation Catalog lines 1176-1195.
bucket.deposits.first_slice,demandability,par|notice|gated|fee_bearing|term_locked,NONE,Representation Catalog lines 1176-1195.
bucket.deposits.first_slice,owner_class,typed_account_owner,NONE,Representation Catalog lines 1176-1195.
bucket.deposits.first_slice,payment_access,named_rail,NONE,Representation Catalog lines 1176-1195.
bucket.deposits.first_slice,legal_conditions,account_level,NONE,Representation Catalog lines 1176-1195.
bucket.money_fund_share.first_slice,redemption_horizon,on_demand|notice,Initializer must provide effective-period numeric cutoffs.,Representation Catalog lines 1176-1195.
bucket.money_fund_share.first_slice,gate_window,none|active,NONE,Representation Catalog lines 1176-1195.
bucket.money_fund_share.first_slice,nav_terms,floating_or_stable,NONE,Representation Catalog lines 1176-1195.
bucket.money_fund_share.first_slice,portfolio_liquidity_cohort,typed,NONE,Representation Catalog lines 1176-1195.
bucket.money_fund_share.first_slice,currency,USD,NONE,Representation Catalog lines 1176-1195.
bucket.money_fund_share.first_slice,fund_class,named_or_residual_cohort,NONE,Representation Catalog lines 1176-1195.
bucket.money_fund_share.first_slice,investor_class,typed,NONE,Representation Catalog lines 1176-1195.
bucket.repo.first_slice,funding_term,overnight|open|term,Initializer must provide effective-period numeric cutoffs.,Representation Catalog lines 1176-1195.
bucket.repo.first_slice,cash_counterparty_class,lender|borrower,NONE,Representation Catalog lines 1176-1195.
bucket.repo.first_slice,settlement_path,bilateral|tri_party|sponsored|gcf_cleared,NONE,Representation Catalog lines 1176-1195.
bucket.repo.first_slice,collateral_bucket,treasury_bucket,NONE,Representation Catalog lines 1176-1195.
bucket.repo.first_slice,haircut_source,named_schedule,NONE,Representation Catalog lines 1176-1195.
bucket.repo.first_slice,margin_regime,contractual_marking,NONE,Representation Catalog lines 1176-1195.
bucket.repo.first_slice,counterparty_class,named_or_residual_cohort,NONE,Representation Catalog lines 1176-1195.
bucket.loans.first_slice,remaining_maturity_band,short|intermediate|long,Initializer must provide effective-period numeric cutoffs.,Representation Catalog lines 1176-1195.
bucket.loans.first_slice,credit_state,performing|delinquent|nonperforming|defaulted|restructured,NONE,Representation Catalog lines 1176-1195.
bucket.loans.first_slice,borrower_class,sector_or_cohort,NONE,Representation Catalog lines 1176-1195.
bucket.loans.first_slice,security_class,secured|unsecured,NONE,Representation Catalog lines 1176-1195.
bucket.loans.first_slice,collateral_class,typed_or_none,NONE,Representation Catalog lines 1176-1195.
bucket.loans.first_slice,rate_form,fixed|floating,NONE,Representation Catalog lines 1176-1195.
bucket.loans.first_slice,currency,USD,NONE,Representation Catalog lines 1176-1195.
bucket.loans.first_slice,claim_rank,contractual,NONE,Representation Catalog lines 1176-1195.
bucket.equity.first_slice,maturity,null,NONE,Representation Catalog lines 1176-1195.
bucket.equity.first_slice,issuer_cohort,modeled_institution,NONE,Representation Catalog lines 1176-1195.
bucket.equity.first_slice,claim_rank,common|preferred,NONE,Representation Catalog lines 1176-1195.
bucket.equity.first_slice,claim_form,tradable|accounting_equity,NONE,Representation Catalog lines 1176-1195.
bucket.equity.first_slice,liquidity_class,market_or_nonmarket,NONE,Representation Catalog lines 1176-1195.
bucket.futures.first_slice,underlying_duration_bucket,treasury_duration,NONE,Representation Catalog lines 1176-1195.
bucket.futures.first_slice,expiry_window,near|deferred,Initializer must provide effective-period numeric cutoffs.,Representation Catalog lines 1176-1195.
bucket.futures.first_slice,basis_exposure,cash_futures,NONE,Representation Catalog lines 1176-1195.
bucket.futures.first_slice,deliverability_class,aggregate_basket,NONE,Representation Catalog lines 1176-1195.
bucket.futures.first_slice,liquidity_limit_class,standard,NONE,Representation Catalog lines 1176-1195.
bucket.futures.first_slice,clearing_exposure,ccp_and_member,NONE,Representation Catalog lines 1176-1195.
bucket.futures.first_slice,margin_regime,initial_and_variation,NONE,Representation Catalog lines 1176-1195.
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/fed_treasury/instrument_buckets.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/fed_treasury/instrument_buckets.csv
size_bytes: 2277
mode_octal: "0644"
modified_at_utc: 2026-09-04T04:02:02.538614Z
sha256: c167fce19cb230a8a996cef85bab974f883debadcc40f44aefadd11dd6e41963
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
bucket_id,instrument_code,display_name,research_status,uncertainty_notes,provenance
bucket.treasury_bill.first_slice,type.instrument_family.treasury_bill,Treasury bills first-slice archetype,researched,Structural archetype only; numeric thresholds remain initializer inputs.,Representation Catalog lines 1168-1195.
bucket.treasury_note.first_slice,type.instrument_family.treasury_note,Treasury notes first-slice archetype,researched,Structural archetype only; numeric thresholds remain initializer inputs.,Representation Catalog lines 1168-1195.
bucket.treasury_bond.first_slice,type.instrument_family.treasury_bond,Treasury bonds first-slice archetype,researched,Structural archetype only; numeric thresholds remain initializer inputs.,Representation Catalog lines 1168-1195.
bucket.reserves.first_slice,type.instrument_family.reserves,Reserves first-slice archetype,researched,Structural archetype only; numeric thresholds remain initializer inputs.,Representation Catalog lines 1168-1195.
bucket.deposits.first_slice,type.instrument_family.deposits,Deposits first-slice archetype,researched,Structural archetype only; numeric thresholds remain initializer inputs.,Representation Catalog lines 1168-1195.
bucket.money_fund_share.first_slice,type.instrument_family.money_fund_share,Money-fund shares first-slice archetype,researched,Structural archetype only; numeric thresholds remain initializer inputs.,Representation Catalog lines 1168-1195.
bucket.repo.first_slice,type.instrument_family.repo,Repo first-slice archetype,researched,Structural archetype only; numeric thresholds remain initializer inputs.,Representation Catalog lines 1168-1195.
bucket.loans.first_slice,type.instrument_family.loans,Loans first-slice archetype,researched,Structural archetype only; numeric thresholds remain initializer inputs.,Representation Catalog lines 1168-1195.
bucket.equity.first_slice,type.instrument_family.equity,Equity first-slice archetype,researched,Structural archetype only; numeric thresholds remain initializer inputs.,Representation Catalog lines 1168-1195.
bucket.futures.first_slice,type.instrument_family.futures,Futures first-slice archetype,researched,Structural archetype only; numeric thresholds remain initializer inputs.,Representation Catalog lines 1168-1195.
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/fed_treasury/instrument_families.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/fed_treasury/instrument_families.csv
size_bytes: 22062
mode_octal: "0644"
modified_at_utc: 2026-09-04T04:02:02.538181Z
sha256: 1f1ecd59374958241a09ff33f750e2a1c27183a55b41e3d6644af834d2746dea
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
instrument_code,display_name,dependency_cut,required_channel,duration,collateral_role,settlement_role,demandability,credit_state,currency,quantity_model,contingency,liquidity,seniority,priority,rollover,rate,convertibility,margin,counterparty_exposure,research_status,uncertainty_notes,provenance
type.instrument_family.treasury_bill,Treasury bills,Circuit,"Short-duration supply, cash-management demand, and general collateral; bills are not required for the cash/futures basis leg itself","Scalar for the fixed discount cash flow, keyed to remaining maturity and yield state","Secured by: no. Usable as: yes, under a named facility, repo, or CCP eligibility schedule and haircut source",Not final money. The security leg settles through the applicable securities-settlement system; the cash leg requires settlement money,Not redeemable on demand; principal is due at contractual maturity,"Performing, defaulted, or restructured where the scenario permits sovereign impairment",Required denomination on every position,"Principal/par amount; market value; short-rate, liquidity, collateral, and issuer exposure",null,"Market depth, executable size, bid/ask, and price-impact state by bill bucket",null; no modeled corporate capital-structure ladder,"null; contractual payment schedule applies, but no separate resolution waterfall in the first slice",High-frequency issuer refinancing/maturity-supply exposure and holder reinvestment exposure by bill bucket,Discount cash-flow form,null,null for the cash security; financing margin belongs to repo or another financing contract,"U.S. Treasury issuer exposure; trading counterparty ends at settlement, while custodian/settlement exposure remains operational state",researched,Numerical initialization and response parameters are not part of the family contract.,Representation Catalog lines 1088-1166.
type.instrument_family.treasury_note,Treasury notes,Core,"Cash leg, benchmark duration supply, dealer inventory, collateral value, forced sale, and note-futures basis","Scalar for fixed coupon cash flows, keyed to remaining maturity and yield state","Secured by: no. Usable as: yes, under a named facility, repo, or CCP eligibility schedule and haircut source",Not final money. The security leg settles through the applicable securities-settlement system; the cash leg requires settlement money,Not redeemable on demand; principal is due at contractual maturity,"Performing, defaulted, or restructured where the scenario permits sovereign impairment",Required denomination on every position,"Principal/par amount; market value; key-rate duration, yield, liquidity, basis, and issuer exposure",null,"Market depth, executable size, bid/ask, and price-impact state by note bucket",null; no modeled corporate capital-structure ladder,"null; contractual payment schedule applies, but no separate resolution waterfall in the first slice",Issuer refinancing/maturity-supply exposure and holder reinvestment exposure by note bucket,Fixed coupon cash-flow form,null,null for the cash security; financing margin belongs to repo or another financing contract,"U.S. Treasury issuer exposure; trading counterparty ends at settlement, while custodian/settlement exposure remains operational state",researched,Numerical initialization and response parameters are not part of the family contract.,Representation Catalog lines 1088-1166.
type.instrument_family.treasury_bond,Treasury bonds,Core,"Long-duration cash leg, dealer inventory, collateral value, forced sale, and bond-futures basis","Scalar for fixed long-dated coupon cash flows, keyed to remaining maturity and yield state","Secured by: no. Usable as: yes, under a named facility, repo, or CCP eligibility schedule and haircut source",Not final money. The security leg settles through the applicable securities-settlement system; the cash leg requires settlement money,Not redeemable on demand; principal is due at contractual maturity,"Performing, defaulted, or restructured where the scenario permits sovereign impairment",Required denomination on every position,"Principal/par amount; market value; long-duration, yield, liquidity, basis, and issuer exposure",null,"Market depth, executable size, bid/ask, and price-impact state by bond bucket",null; no modeled corporate capital-structure ladder,"null; contractual payment schedule applies, but no separate resolution waterfall in the first slice",Issuer refinancing/maturity-supply exposure and holder reinvestment exposure by bond bucket,Fixed long-dated coupon cash-flow form,null,null for the cash security; financing margin belongs to repo or another financing contract,"U.S. Treasury issuer exposure; trading counterparty ends at settlement, while custodian/settlement exposure remains operational state",researched,Numerical initialization and response parameters are not part of the family contract.,Representation Catalog lines 1088-1166.
type.instrument_family.tips,TIPS,Long tail,Inflation-linked duration substitution and real-yield information,"State-dependent function over remaining maturity, real yields, and indexation state","Secured by: no. Usable as: yes, under the applicable eligibility schedule and haircut source",Same securities leg/cash leg distinction as nominal Treasuries,Not redeemable on demand; indexed principal is due under the instrument terms,"Performing, defaulted, or restructured where applicable",Required denomination plus named inflation-index reference,"Indexed principal/par amount; market value; real-rate, inflation, liquidity, and issuer exposure",null,Separate market-depth and price-impact state from nominal Treasuries,null for the same reason as nominal Treasuries,null in the first-slice resolution model,Issuer refinancing and holder reinvestment exposure by maturity bucket,Fixed real coupon plus principal/cash-flow indexation to a named PublishedReference,null; indexation is not conversion,null for the cash security,U.S. Treasury issuer plus index-publication and settlement dependencies,researched,Numerical initialization and response parameters are not part of the family contract.,Representation Catalog lines 1088-1166.
type.instrument_family.reserves,Reserves,Core,"Final cash settlement for repo, margin, Treasury trades, and facility operations",Overnight/administered-rate balance; no fixed cash-flow duration schedule,Secured by: no. Usable as: no; the balance is settlement money rather than pledged collateral,Final settlement asset within the issuing central-bank payment system,Transferable on demand at par by an eligible account holder,null; a reserve balance has no loan-performance lifecycle,Required denomination,"Account balance; par/book value; issuing-central-bank, settlement, and rate exposure",null,Immediately transferable subject to account and system availability,null,"Payment-queue priority belongs to the settlement system, not the reserve instrument",null; no contractual maturity to refinance,Administered or tiered rate set by the issuing central bank,"null; withdrawal or payment is settlement/demandability, not conversion",null,Issuing Reserve Bank/central bank and payment-system operational exposure,researched,Numerical initialization and response parameters are not part of the family contract.,Representation Catalog lines 1088-1166.
type.instrument_family.deposits,Deposits,Circuit,"Bank funding, payment outflows, and demandable cash claims once bank liquidity is endogenous","State-dependent by account term, repricing convention, and withdrawal behavior",Secured by: normally no. Usable as: only where an explicit eligibility schedule admits the claim,Transferable commercial-bank money on the issuer's ledger and connected payment rails; interbank finality uses reserves,"On demand at par, notice, gated or fee-bearing, or term-locked, as account terms specify","null for the deposit claim itself; issuer impairment is counterparty state, not loan delinquency",Required denomination,"Account balance; par claim/liability value and any market carrying value; bank, liquidity, and rate exposure",null,"Withdrawal and transfer liquidity determined by demandability, bank operations, and payment-system state",Set by account terms and effective LegalInstrument clauses; not fixed by the family,"Set by payment, insolvency, and resolution law at account level",Term-deposit maturity or repricing; demand deposits have no contractual refinancing event,"Fixed, floating, administered, promotional, or referenced rate by account terms",null; withdrawal/payment is demandability,null for the deposit claim,"Issuing bank, account custodian, and payment-path exposure",researched,Numerical initialization and response parameters are not part of the family contract.,Representation Catalog lines 1088-1166.
type.instrument_family.money_fund_share,Money-fund shares,Circuit,Investor redemption and the repo cash-supplier channel once money funds are endogenous rather than an adapter,State-dependent function over portfolio duration and redemption terms,Secured by: no. Usable as: no by default,Not settlement money; redemption produces a payment claim,"Redeemable at NAV, subject to applicable notice, fee, or gate conditions",null; portfolio impairments remain on the assets held by the fund,Required denomination/NAV currency,"Share units or subscribed principal; NAV market value; fund, portfolio, and redemption exposure",null,"Redemption capacity, portfolio liquidity, and any secondary-market depth","Residual beneficial claim on the fund portfolio, subject to governing law and share class",Redemption and liquidation ordering comes from fund terms and legal conditions,null; redemption pressure replaces issuer refinancing risk,Variable portfolio income/distribution yield; no promised deposit rate,null; redemption at NAV is demandability,null for the share,"Fund, sponsor where legally relevant, custodian, and portfolio exposure",researched,Numerical initialization and response parameters are not part of the family contract.,Representation Catalog lines 1088-1166.
type.instrument_family.repo,Repo,Core,"Leveraged funding, collateral encumbrance/reuse rights, haircut and margin calls, rollover, and forced deleveraging",Contractual remaining term; open and callable forms carry state-dependent effective duration,"Secured by: yes, with identified collateral, eligibility, valuation, and haircut source. Usable as: null; received collateral, not the repo claim, may be reused under account and agreement rules",Not settlement money; cash and collateral legs settle in their declared systems,Open/callable or fixed-term according to the contract; not a par-redemption claim,"Performing, failed-to-settle, defaulted, or restructured/closed out as contract state requires",Required denomination for the cash leg; collateral positions retain their own currencies,"Cash principal; accrued/market value; collateral, haircut, rate, maturity, and counterparty exposure",null,"Funding-market depth, collateral liquidity, and executable capacity by contract bucket",Secured claim to the extent collateral and close-out rights are effective,Collateral realization and close-out priority from agreement and law,"Required: overnight, open, and term contracts mature, call, or renew","Fixed or floating repo rate, optionally tied to a PublishedReference",null,Initial haircut/margin and variation or mark-to-market calls where terms require them,"Bilateral counterparty, tri-party custodian, or novated CCP/clearing-member exposure, kept distinct",researched,Numerical initialization and response parameters are not part of the family contract.,Representation Catalog lines 1088-1166.
type.instrument_family.loans,Loans,Circuit,Transmission from bank funding and capital constraints into lending posture and aggregate credit conditions,"Scalar for fixed bullet cash flows or state-dependent for amortization, prepayment, floating rates, and default",Secured by: yes or no by contract. Usable as: only under a facility or private eligibility schedule and haircut source,Not settlement money; repayments settle separately,"Not redeemable by the lender; borrower repayment, prepayment, or contractual call governs","Performing, delinquent, non-performing, defaulted, or restructured",Required denomination,"Principal or commitment amount; carrying/fair value; borrower, collateral, rate, and credit exposure",null; contingent lending belongs to the guarantees/credit-lines family until drawn,Sale/securitization liquidity and refinancing availability by loan bucket,Secured/unsecured and subordinated rank from contract,Payment and recovery order from contract and law,"Required where borrower refinance, lender renewal, or maturity-wall risk exists","Fixed, floating, administered, or PublishedReference-linked, including reset terms",Optional only where the loan contract expressly converts into another claim; otherwise null,null for the loan itself unless a separate collateral/margin agreement requires calls,"Borrower, guarantor, servicer, syndicate, and protection provider where represented",researched,Numerical initialization and response parameters are not part of the family contract.,Representation Catalog lines 1088-1166.
type.instrument_family.agency_mbs,Agency MBS,Long tail,"Convexity hedging, alternative collateral, and duration substitution","State-dependent function over rates, prepayment, pool/vintage state, and embedded optionality","Secured by: mortgage-pool claims and any agency guarantee structure. Usable as: conditional on facility, repo, or CCP schedule and haircut source",Not settlement money; security and cash legs settle separately,Not redeemable on demand; principal arrives through scheduled amortization and prepayment,"Performing, delinquent, non-performing, defaulted, or modified at underlying-pool level, with guarantee performance separate",Required denomination,"Current principal balance; market value; duration/convexity, prepayment, pool, agency, and liquidity exposure",null,Market depth and price impact by duration/convexity/liquidity bucket,Pool/tranche and guarantee structure where applicable,Cash-flow waterfall and guarantee rules where applicable,null for the holder; underlying refinancing appears as prepayment state,"Pass-through coupon and servicing/guarantee terms, with any index references named",null,null for settled cash MBS; forward/TBA obligations may carry agreement-based margin,"Agency/guarantor, pool, servicer, seller, custodian, and clearing counterparty as applicable",researched,Numerical initialization and response parameters are not part of the family contract.,Representation Catalog lines 1088-1166.
type.instrument_family.corporate_bonds,Corporate bonds,Long tail,Credit-spread contagion and dealer-balance-sheet competition,"Scalar for fixed non-callable cash flows or state-dependent for calls, puts, and other optionality",Secured by: secured or unsecured by issue terms. Usable as: conditional on an eligibility schedule and haircut source,Not settlement money; security and cash legs settle separately,"Not redeemable on demand; maturity, put, or call terms govern","Performing, defaulted, or restructured",Required denomination,"Principal/par amount; market value; issuer, spread, duration, liquidity, and optionality exposure",null,"Market depth and price impact by maturity, rating/credit, and issue-liquidity bucket","Secured, senior, subordinated, or other contract rank",Payment and recovery order from indenture and law,Issuer refinancing and holder maturity/reinvestment exposure,"Fixed, floating, step, or PublishedReference-linked coupon",Optional conversion into equity or another claim only where issue terms grant it,null for the cash bond; financing margin belongs to its financing contract,"Issuer, guarantor, trustee, custodian, and settlement exposure as applicable",researched,Numerical initialization and response parameters are not part of the family contract.,Representation Catalog lines 1088-1166.
type.instrument_family.equity,Equity,Core,"Dealer, bank, fund, and clearing-member loss absorption and balance-sheet/risk-limit capacity; accounting equity is distinct from a tradable equity position",null; no contractual maturity or promised cash-flow duration,Secured by: no. Usable as: conditional on a lender or CCP collateral schedule; otherwise no,Not settlement money; share and cash legs settle separately,Not redeemable from the issuer by default; issuer repurchase is a separate action,null; issuer distress changes value but is not an instrument credit-state ladder,Required trading and accounting currency,"Notional: null; share units and market value; issuer, factor, liquidity, and voting/control exposure",null,Market depth and price impact by issuer/cohort and share class,Residual claim; preferred or class rank only when terms require it,Residual distribution/liquidation order from charter and law,null; no maturity wall,"null; dividends are discretionary distributions, not a contractual rate",Optional class conversion only where charter terms grant it; otherwise null,null for settled cash equity; broker financing margin belongs to the financing contract,"Issuer, custodian, broker, exchange/CCP, and settlement exposure as applicable",researched,Numerical initialization and response parameters are not part of the family contract.,Representation Catalog lines 1088-1166.
type.instrument_family.swaps,Swaps,Long tail,Alternative duration hedging and cleared-margin demand,"Contractual remaining tenor, with state-dependent exposure where legs, optionality, or curves change",Secured by: only through a CSA or clearing arrangement. Usable as: no; posted collateral remains a separate position,"Not settlement money; periodic, termination, and margin cash flows settle separately","Not redeemable; termination, break, assignment, or novation follows contract terms","Performing, disputed, terminated, or defaulted under contract/close-out state",Required for every leg and settlement amount,"Contract notional by leg; current replacement/market value; gross, netted, rate, currency, and counterparty exposure","null; conditional leg payments are payoff terms, not undrawn commitments","Exit, novation, compression, and replacement liquidity","Close-out claim rank depends on collateral, netting agreement, and law","Netting, collateral, and default-management rules determine payment order","Maturity, reset, compression, replacement, or novation exposure","Typed legs: fixed, floating, inflation, currency, or other named reference, each linked to its PublishedReference where applicable","null; exchange of legs is the payoff, not a conversion right","Initial and variation margin under CSA or CCP rules, with collateral source and settlement schedule",Bilateral under ISDA/CSA or novated to a CCP through a clearing member; gross and net exposure both retained,researched,Numerical initialization and response parameters are not part of the family contract.,Representation Catalog lines 1088-1166.
type.instrument_family.futures,Futures,Core,"Short futures leg, contract notional, variation margin, basis exposure, and CCP/clearing-member path","Remaining time to expiry or delivery, with state-dependent exposure to the underlying risk factor",Secured by: null in the collateral sense; performance is supported through margin and the clearing framework. Usable as: no,Not settlement money; variation settlement and any delivery settle in declared systems,"Not redeemable; expiry, close-out, or delivery governs","Performing, in default management, closed, expired, or delivered as clearing state requires",Required contract quote and settlement currency,"Contract notional; current marked value/variation balance; delta, duration, basis, and CCP exposure",null,"Order-book depth, position limits, and liquidation impact by contract bucket",null as an instrument rank; claims on a defaulting member follow clearing rules,"CCP default waterfall and settlement ordering, not a corporate coupon rank",Expiry and roll into a later contract month,No cash coupon; contract price implies or carries exposure to the named underlying/reference,null,"Initial and variation margin, intraday calls, settlement schedule, and margin authority","CCP and clearing-member exposure, plus delivery counterparty only where physical delivery survives clearing",researched,Numerical initialization and response parameters are not part of the family contract.,Representation Catalog lines 1088-1166.
type.instrument_family.guarantees_credit_lines,Guarantees and credit lines,Long tail,Contingent liquidity calls and official/private backstops,Commitment horizon while undrawn; drawn exposure takes the duration form of the resulting claim,Secured by: optional under the commitment terms. Usable as: no,Not settlement money; a draw or guarantee payment creates a separate settlement obligation,Draw may be on demand or condition-triggered within the commitment; the undrawn promise is not redeemable,"Performing commitment while undrawn; after draw, the resulting claim carries its own credit state","Required commitment, draw, and settlement currency","Committed notional/limit; fair value or provision; expected, current, drawn, guarantor, and beneficiary exposure","Required: undrawn, partially drawn, drawn, expired, cancelled, or called, with typed trigger conditions",Transferability is normally limited; draw and funding availability dominate,Rank of the drawn or paid claim follows commitment and resulting instrument terms,"Draw, reimbursement, and recovery order follows contract and law","Commitment expiry, renewal, cancellation, and refinancing of drawn balances",Commitment fee while undrawn and typed rate on any drawn claim,null; draw is a contingency transition,null unless the commitment agreement separately requires collateral or margin,"Guarantor/lender, beneficiary/borrower, reference obligor, syndicate, and reimbursement counterparty as applicable",researched,Numerical initialization and response parameters are not part of the family contract.,Representation Catalog lines 1088-1166.
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/fed_treasury/presentation_refs.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/fed_treasury/presentation_refs.csv
size_bytes: 675
mode_octal: "0644"
modified_at_utc: 2026-09-04T04:03:06.727536Z
sha256: 80330b9fbc6bc0cb74ab8cfbc5fba569fddb23d29e3abfd9d799da1a2dfb54ac
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
catalog_id,asset_role,asset_path,provenance
person.us.ben_bernankey,headshot,assets/headshots/ben-bernankey.png,User-selected presentation asset.
person.us.alan_greenspaniel,headshot,assets/headshots/alan-greenspaniel-v2.png,User-selected presentation asset.
person.us.janet_jackrabbit,headshot,assets/headshots/janet-jackrabbit-v2.png,User-selected presentation asset.
person.us.jerome_owl,headshot,assets/headshots/jerome-owl-v2.png,User-selected presentation asset.
person.us.kevin_boarsh,headshot,assets/headshots/kevin-boarsh-v2.png,User-selected presentation asset.
person.us.paul_vulture,headshot,assets/headshots/paul-vulture-v2.png,User-selected presentation asset.
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/fed_treasury/profile_required_offices.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/fed_treasury/profile_required_offices.csv
size_bytes: 509
mode_octal: "0644"
modified_at_utc: 2026-09-04T04:03:06.727641Z
sha256: 167347027482ab01b4e30f26bbaa1f27b74f0f597f66843a024fcd7027015426
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
profile_id,office_id,requirement,provenance
profile.early_2006.bernankey,office.us.federal_reserve.board_chair,The player must effectively hold this Chair office in the profile period.,Federal Reserve History: https://www.federalreservehistory.org/people/federal-reserve-chair
profile.early_2006.bernankey,office.us.federal_reserve.fomc_chair,The player must effectively hold this Chair office in the profile period.,Federal Reserve History: https://www.federalreservehistory.org/people/federal-reserve-chair
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase1/entities.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/mvp_phase1/entities.csv
size_bytes: 672
mode_octal: "0644"
modified_at_utc: 2026-09-04T17:29:55.202035Z
sha256: 35ea26b64d8cab8ba57c6fbdec70a290e785557f14a72b96f2f537c712156f66
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
catalog_id,display_name,entry_class,instance_of,identity_clade,domain,selectable_in_manifest,definition_version,cognition_class,default_fidelity,research_status,completeness_state,fit,fallback_entry_id,residual_counterpart_id,uncertainty_notes,provenance
adapter.macro.us.broad,Broad U.S. macro adapter,instance,type.boundary_adapter.default,BoundaryAdapter,macro_boundary,false,1,none,MECHANICAL_OR_ADAPTER,sketched,probe_complete,fits,UNKNOWN,NONE,Phase 1 owns locked hidden conditions and release configuration; broader macro transmission remains outside this slice.,12-structure-outline-bernankey-mvp-cycle.md:81-84;11-design-discussion-bernankey-mvp-slice.md:299-340
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase1/entity_authority_sources.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/mvp_phase1/entity_authority_sources.csv
size_bytes: 565
mode_octal: "0644"
modified_at_utc: 2026-09-04T17:29:55.202348Z
sha256: 3956369270793ef93ef4c6a79b959702d0a815bac6c83b1bfd3d4a4e0714e247
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
catalog_id,authority_source_id,authority_source_kind,provenance
adapter.macro.us.broad,profile.early_2006.bernankey,scenario_boundary_contract,12-structure-outline-bernankey-mvp-cycle.md:174-177
office.us.federal_reserve.board_chair,NONE,non_authorizing_phase_1_identity,12-structure-outline-bernankey-mvp-cycle.md:174-177
office.us.federal_reserve.fomc_chair,NONE,non_authorizing_phase_1_identity,12-structure-outline-bernankey-mvp-cycle.md:174-177
reference.us.bls.cpi,institution.us.bls,publishing_institution,12-structure-outline-bernankey-mvp-cycle.md:174-177
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase1/entity_fallback_contracts.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/mvp_phase1/entity_fallback_contracts.csv
size_bytes: 598
mode_octal: "0644"
modified_at_utc: 2026-09-04T17:29:55.202662Z
sha256: b027872581379e05da3dcc1d03164687da2805c2d65c46c0e9b14493f302e1fa
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
catalog_id,preserves_or_loses,boundary_item,provenance
adapter.macro.us.broad,lose,scenario.fallback.FAIL,12-structure-outline-bernankey-mvp-cycle.md:128-143
office.us.federal_reserve.board_chair,lose,scenario.fallback.FAIL,12-structure-outline-bernankey-mvp-cycle.md:128-143
office.us.federal_reserve.fomc_chair,lose,scenario.fallback.FAIL,12-structure-outline-bernankey-mvp-cycle.md:128-143
person.us.ben_bernankey,lose,scenario.fallback.FAIL,12-structure-outline-bernankey-mvp-cycle.md:128-143
reference.us.bls.cpi,lose,scenario.fallback.FAIL,12-structure-outline-bernankey-mvp-cycle.md:128-143
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase1/owned_state.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/mvp_phase1/owned_state.csv
size_bytes: 1311
mode_octal: "0644"
modified_at_utc: 2026-09-04T17:29:55.203172Z
sha256: 2472c5bd5072f4047010a1f95e831eafbd5ae5a2e14e602cd5c5344f8e751cf7
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
state_id,owner_id,state_kind,value_domain,unit,conserved,witness_kind,completeness_state,provenance
state.adapter.macro.us.broad.hidden_state,adapter.macro.us.broad,condition,hidden inflation labor housing and release configuration,NONE,false,typed adapter transition and release witness,probe_complete,12-structure-outline-bernankey-mvp-cycle.md:81-84
state.office.us.federal_reserve.board_chair.tenure,office.us.federal_reserve.board_chair,authority,holder term and access identity,NONE,false,effective office-holding witness,probe_complete,12-structure-outline-bernankey-mvp-cycle.md:174-177
state.office.us.federal_reserve.fomc_chair.tenure,office.us.federal_reserve.fomc_chair,authority,holder term and access identity,NONE,false,effective office-holding witness,probe_complete,12-structure-outline-bernankey-mvp-cycle.md:174-177
state.person.us.ben_bernankey.cognition,person.us.ben_bernankey,record,private belief memory goal and plan ledger,NONE,false,recipient-owned cognition transition witness,probe_complete,12-structure-outline-bernankey-mvp-cycle.md:174-177
state.reference.us.bls.cpi.publication,reference.us.bls.cpi,publication,current value and publication revision history,index_points,false,measurement and publication witness,probe_complete,12-structure-outline-bernankey-mvp-cycle.md:81-84
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase1/owned_state_transitions.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/mvp_phase1/owned_state_transitions.csv
size_bytes: 607
mode_octal: "0644"
modified_at_utc: 2026-09-04T17:29:55.203441Z
sha256: 44c8372f4872e1021f0e7a01c3c6d12ef47d0f717a5d43dedcd1a48db59cceab
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
state_id,transition_kind,provenance
state.adapter.macro.us.broad.hidden_state,publish_release,12-structure-outline-bernankey-mvp-cycle.md:157-172
state.office.us.federal_reserve.board_chair.tenure,legal_transition,12-structure-outline-bernankey-mvp-cycle.md:146-155
state.office.us.federal_reserve.fomc_chair.tenure,legal_transition,12-structure-outline-bernankey-mvp-cycle.md:146-155
state.person.us.ben_bernankey.cognition,evidence_integration,12-structure-outline-bernankey-mvp-cycle.md:157-172
state.reference.us.bls.cpi.publication,publish_reference,12-structure-outline-bernankey-mvp-cycle.md:146-155
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase1/period_variants.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/mvp_phase1/period_variants.csv
size_bytes: 1120
mode_octal: "0644"
modified_at_utc: 2026-09-04T17:29:55.202915Z
sha256: 6589d0ba1c513e11dd6936a1c28af3ed36de7b5331d6276223a1b9a79b813169
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
catalog_id,variant_id,effective_period,authority_or_eligibility_ref,uncertainty_notes,provenance
adapter.macro.us.broad,variant.2006,2006-02-01/2006-12-31,profile.early_2006.bernankey,Broad macro behavior is locked by the scenario initialization and release tape.,12-structure-outline-bernankey-mvp-cycle.md:102-143
office.us.federal_reserve.board_chair,variant.2006,2006-02-01/2006-12-31,rel.holds.ben_bernankey.board_chair,Action authority is intentionally deferred to Phase 2.,12-structure-outline-bernankey-mvp-cycle.md:102-143
office.us.federal_reserve.fomc_chair,variant.2006,2006-02-01/2006-12-31,rel.holds.ben_bernankey.fomc_chair,Action authority is intentionally deferred to Phase 2.,12-structure-outline-bernankey-mvp-cycle.md:102-143
person.us.ben_bernankey,variant.2006,2006-02-01/2006-12-31,profile.early_2006.bernankey,The playable identity is fixed to the early-2006 profile.,12-structure-outline-bernankey-mvp-cycle.md:102-143
reference.us.bls.cpi,variant.2006,2006-02-01/2006-12-31,institution.us.bls,Release details are locked by the scenario tape.,12-structure-outline-bernankey-mvp-cycle.md:102-143
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase1/profile_catalog_roles.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/mvp_phase1/profile_catalog_roles.csv
size_bytes: 376
mode_octal: "0644"
modified_at_utc: 2026-09-04T17:29:55.203673Z
sha256: 9f03fd4e3933601f30b51f67923800453fb6800194a50514560931eb30eb8827
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
profile_id,catalog_id,profile_role,candidate_provider_entry_id,activation_requirement,rationale,provenance
profile.early_2006.bernankey,adapter.macro.us.broad,boundary_candidate,adapter.macro.us.broad,Selected only by the bounded MVP manifest with locked opening state and release tape.,Phase 1 broad macro boundary provider.,12-structure-outline-bernankey-mvp-cycle.md:65-67
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase2/entities.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/mvp_phase2/entities.csv
size_bytes: 4183
mode_octal: "0644"
modified_at_utc: 2026-09-04T17:52:35.664930Z
sha256: 538610a144decd03b52f9e2cbc7584f5a07cdef726469643a8bdebcaaa069888
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
catalog_id,display_name,entry_class,instance_of,identity_clade,domain,selectable_in_manifest,definition_version,cognition_class,default_fidelity,research_status,completeness_state,fit,fallback_entry_id,residual_counterpart_id,uncertainty_notes,provenance
adapter.market.us.treasury_demand.phase2,Phase 2 Treasury demand boundary adapter,instance,type.boundary_adapter.default,BoundaryAdapter,markets,false,1,none,MECHANICAL_OR_ADAPTER,sketched,probe_complete,fits,UNKNOWN,NONE,Returns explicit adapter-sourced acknowledgements without price formation allocation fills or settlement.,12-structure-outline-bernankey-mvp-cycle.md:260-264
legal.us.federal_reserve.act.section_12a,Federal Reserve Act section 12A,instance,type.legal_instrument.default,LegalInstrument,Fed,false,1,none,MECHANICAL_OR_ADAPTER,researched,probe_complete,fits,UNKNOWN,NONE,The scenario stores bounded effective clauses for FOMC direction of open-market operations.,Federal Reserve Act section 12A: https://www.federalreserve.gov/aboutthefed/section12a.htm
legal.us.federal_reserve.act.section_14,Federal Reserve Act section 14,instance,type.legal_instrument.default,LegalInstrument,Fed,false,1,none,MECHANICAL_OR_ADAPTER,researched,probe_complete,fits,UNKNOWN,NONE,The scenario stores bounded effective clauses for Reserve Bank open-market powers.,Federal Reserve Act section 14: https://www.federalreserve.gov/aboutthefed/section14.htm
legal.us.federal_reserve.domestic_authorization.2006,2006 Authorization for Domestic Open Market Operations,instance,type.legal_instrument.default,LegalInstrument,Fed,false,1,none,MECHANICAL_OR_ADAPTER,researched,probe_complete,fits,UNKNOWN,NONE,The MVP models only the delegation required for the selected Desk actions.,Federal Reserve 2006 FOMC historical materials: https://www.federalreserve.gov/monetarypolicy/fomchistorical2006.htm
legal.us.federal_reserve.fomc_rules.section_3,FOMC Rules of Organization section 3,instance,type.legal_instrument.default,LegalInstrument,Fed,false,1,none,MECHANICAL_OR_ADAPTER,researched,probe_complete,fits,UNKNOWN,NONE,The scenario stores the bounded voting-procedure clause effective for the meeting.,FOMC Rules of Organization: https://www.federalreserve.gov/monetarypolicy/files/fomc_rulesorganization.pdf
role_holder.fomc.governor_1,Governor Heron,instance,type.person.default,Person,Fed,false,1,named_cognition,LIMITED_ROLE_HOLDER,sketched,probe_complete,fits,UNKNOWN,NONE,Fictional bounded role-holder with sourced beliefs for the MVP meeting; not a historical identity.,12-structure-outline-bernankey-mvp-cycle.md:226-232
role_holder.fomc.governor_2,Governor Hare,instance,type.person.default,Person,Fed,false,1,named_cognition,LIMITED_ROLE_HOLDER,sketched,probe_complete,fits,UNKNOWN,NONE,Fictional bounded role-holder with sourced beliefs for the MVP meeting; not a historical identity.,12-structure-outline-bernankey-mvp-cycle.md:226-232
record.us.federal_reserve.action_result,Desk Action Result,instance,type.record.default,Record,Fed,false,1,none,MECHANICAL_OR_ADAPTER,sketched,probe_complete,fits,UNKNOWN,NONE,Separates execution status and failure stage from authorization and settlement.,12-structure-outline-bernankey-mvp-cycle.md:195-285
record.us.federal_reserve.authorization_decision,FOMC Authorization Decision,instance,type.record.default,Record,Fed,false,1,none,MECHANICAL_OR_ADAPTER,sketched,probe_complete,fits,UNKNOWN,NONE,Records the authority holder vote status scope effective interval and source proposal.,12-structure-outline-bernankey-mvp-cycle.md:195-285
record.us.federal_reserve.directive,FOMC Desk Directive,instance,type.record.default,Record,Fed,false,1,none,MECHANICAL_OR_ADAPTER,sketched,probe_complete,fits,UNKNOWN,NONE,Certifies the exact Desk effects authorized by a successful Committee vote.,12-structure-outline-bernankey-mvp-cycle.md:195-285
record.us.federal_reserve.stage_receipt,Policy Chain Stage Receipt,instance,type.record.default,Record,Fed,false,1,none,MECHANICAL_OR_ADAPTER,sketched,probe_complete,fits,UNKNOWN,NONE,Identifies proposal authorization execution settlement and observed-effect stages without conflating them.,12-structure-outline-bernankey-mvp-cycle.md:195-285
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase2/entity_authority_sources.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/mvp_phase2/entity_authority_sources.csv
size_bytes: 999
mode_octal: "0644"
modified_at_utc: 2026-09-04T17:57:10.011735Z
sha256: ce8be4c52dd720d0d5eb71ddfb8ba2bd96066ef7c51cf2ef1962949bc9046252
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
catalog_id,authority_source_id,authority_source_kind,provenance
body.us.federal_reserve.fomc,legal.us.federal_reserve.act.section_12a,statute,Federal Reserve Act section 12A: https://www.federalreserve.gov/aboutthefed/section12a.htm
body.us.federal_reserve.fomc,legal.us.federal_reserve.fomc_rules.section_3,rule,FOMC Rules of Organization: https://www.federalreserve.gov/monetarypolicy/files/fomc_rulesorganization.pdf
inst.us.federal_reserve.new_york,legal.us.federal_reserve.act.section_14,statute,Federal Reserve Act section 14: https://www.federalreserve.gov/aboutthefed/section14.htm
inst.us.federal_reserve.new_york,legal.us.federal_reserve.domestic_authorization.2006,delegation,Federal Reserve 2006 FOMC historical materials: https://www.federalreserve.gov/monetarypolicy/fomchistorical2006.htm
office.us.federal_reserve.fomc_chair,legal.us.federal_reserve.fomc_rules.section_3,rule,FOMC Rules of Organization: https://www.federalreserve.gov/monetarypolicy/files/fomc_rulesorganization.pdf
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase2/entity_fallback_contracts.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/mvp_phase2/entity_fallback_contracts.csv
size_bytes: 2296
mode_octal: "0644"
modified_at_utc: 2026-09-04T17:59:57.428549Z
sha256: 74973f4115fae56977de69751cdd839dd2bf380a24d2bae52a638226ace7e77c
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
catalog_id,preserves_or_loses,boundary_item,provenance
adapter.market.us.treasury_demand.phase2,lose,scenario.fallback.FAIL,12-structure-outline-bernankey-mvp-cycle.md:195-285
body.us.federal_reserve.fomc,lose,scenario.fallback.FAIL,12-structure-outline-bernankey-mvp-cycle.md:195-285
inst.us.federal_reserve.board,lose,scenario.fallback.FAIL,12-structure-outline-bernankey-mvp-cycle.md:195-285
inst.us.federal_reserve.new_york,lose,scenario.fallback.FAIL,12-structure-outline-bernankey-mvp-cycle.md:195-285
legal.us.federal_reserve.act.section_12a,lose,scenario.fallback.FAIL,12-structure-outline-bernankey-mvp-cycle.md:195-285
legal.us.federal_reserve.act.section_14,lose,scenario.fallback.FAIL,12-structure-outline-bernankey-mvp-cycle.md:195-285
legal.us.federal_reserve.domestic_authorization.2006,lose,scenario.fallback.FAIL,12-structure-outline-bernankey-mvp-cycle.md:195-285
legal.us.federal_reserve.fomc_rules.section_3,lose,scenario.fallback.FAIL,12-structure-outline-bernankey-mvp-cycle.md:195-285
office.us.federal_reserve.chief_of_staff,lose,scenario.fallback.FAIL,12-structure-outline-bernankey-mvp-cycle.md:195-285
office.us.federal_reserve.governor,lose,scenario.fallback.FAIL,12-structure-outline-bernankey-mvp-cycle.md:195-285
record.us.federal_reserve.action_result,lose,scenario.fallback.FAIL,12-structure-outline-bernankey-mvp-cycle.md:195-285
record.us.federal_reserve.authorization_decision,lose,scenario.fallback.FAIL,12-structure-outline-bernankey-mvp-cycle.md:195-285
record.us.federal_reserve.directive,lose,scenario.fallback.FAIL,12-structure-outline-bernankey-mvp-cycle.md:195-285
record.us.federal_reserve.policy_package,lose,scenario.fallback.FAIL,12-structure-outline-bernankey-mvp-cycle.md:195-285
record.us.federal_reserve.stage_receipt,lose,scenario.fallback.FAIL,12-structure-outline-bernankey-mvp-cycle.md:195-285
role_holder.fomc.governor_1,lose,scenario.fallback.FAIL,12-structure-outline-bernankey-mvp-cycle.md:195-285
role_holder.fomc.governor_2,lose,scenario.fallback.FAIL,12-structure-outline-bernankey-mvp-cycle.md:195-285
schedule.us.federal_reserve.blackout,lose,scenario.fallback.FAIL,12-structure-outline-bernankey-mvp-cycle.md:195-285
schedule.us.federal_reserve.fomc,lose,scenario.fallback.FAIL,12-structure-outline-bernankey-mvp-cycle.md:195-285
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase2/owned_state.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/mvp_phase2/owned_state.csv
size_bytes: 4050
mode_octal: "0644"
modified_at_utc: 2026-09-04T17:57:10.011121Z
sha256: 082447ee1b88f6ba224f27e5191d5de5d3043bce5511203cb25337e7e5b1299c
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
state_id,owner_id,state_kind,value_domain,unit,conserved,witness_kind,completeness_state,provenance
state.adapter.market.us.treasury_demand.phase2.boundary,adapter.market.us.treasury_demand.phase2,clearing_result,explicit adapter label with null price and allocation,NONE,false,boundary adapter receipt,probe_complete,12-structure-outline-bernankey-mvp-cycle.md:260-264
state.body.us.federal_reserve.fomc.procedure,body.us.federal_reserve.fomc,authority,quorum threshold vote dissent and directive history,NONE,false,FOMC decision record,probe_complete,12-structure-outline-bernankey-mvp-cycle.md:246-259
state.inst.us.federal_reserve.board.governance,inst.us.federal_reserve.board,authority,Board governance identity distinct from FOMC and SOMA ownership,NONE,false,legal authority reference,probe_complete,12-structure-outline-bernankey-mvp-cycle.md:195-205
state.inst.us.federal_reserve.new_york.desk_authority,inst.us.federal_reserve.new_york,authority,directive-bounded Desk execution and settlement status,NONE,false,Desk action result,probe_complete,12-structure-outline-bernankey-mvp-cycle.md:260-264
state.legal.us.federal_reserve.act.section_12a.effective_clauses,legal.us.federal_reserve.act.section_12a,authority,effective bounded clause identifiers,NONE,false,legal source citation,probe_complete,Federal Reserve Act section 12A: https://www.federalreserve.gov/aboutthefed/section12a.htm
state.legal.us.federal_reserve.act.section_14.effective_clauses,legal.us.federal_reserve.act.section_14,authority,effective bounded clause identifiers,NONE,false,legal source citation,probe_complete,Federal Reserve Act section 14: https://www.federalreserve.gov/aboutthefed/section14.htm
state.legal.us.federal_reserve.domestic_authorization.2006.effective_clauses,legal.us.federal_reserve.domestic_authorization.2006,authority,effective bounded clause identifiers,NONE,false,legal source citation,probe_complete,Federal Reserve 2006 FOMC historical materials: https://www.federalreserve.gov/monetarypolicy/fomchistorical2006.htm
state.legal.us.federal_reserve.fomc_rules.section_3.effective_clauses,legal.us.federal_reserve.fomc_rules.section_3,authority,effective bounded clause identifiers,NONE,false,legal source citation,probe_complete,FOMC Rules of Organization: https://www.federalreserve.gov/monetarypolicy/files/fomc_rulesorganization.pdf
state.office.us.federal_reserve.chief_of_staff.tenure,office.us.federal_reserve.chief_of_staff,authority,limited role-holder effective period,NONE,false,office tenure record,probe_complete,12-structure-outline-bernankey-mvp-cycle.md:226-232
state.office.us.federal_reserve.governor.mvp_roster,office.us.federal_reserve.governor,record,bounded FOMC participant role-holder roster,NONE,false,meeting roster record,probe_complete,12-structure-outline-bernankey-mvp-cycle.md:226-232
state.record.us.federal_reserve.policy_package.records,record.us.federal_reserve.policy_package,record,prepared submitted and narrowed package history,NONE,false,policy package record,probe_complete,12-structure-outline-bernankey-mvp-cycle.md:233-245
state.role_holder.fomc.governor_1.cognition,role_holder.fomc.governor_1,record,private bounded beliefs with source ledger,NONE,false,private cognition transition witness,probe_complete,12-structure-outline-bernankey-mvp-cycle.md:226-232
state.role_holder.fomc.governor_2.cognition,role_holder.fomc.governor_2,record,private bounded beliefs with source ledger,NONE,false,private cognition transition witness,probe_complete,12-structure-outline-bernankey-mvp-cycle.md:226-232
state.schedule.us.federal_reserve.blackout.current,schedule.us.federal_reserve.blackout,schedule,derived communication blackout interval,NONE,false,published schedule derivation,probe_complete,12-structure-outline-bernankey-mvp-cycle.md:278-285
state.schedule.us.federal_reserve.fomc.calendar,schedule.us.federal_reserve.fomc,schedule,published meeting occurrence and derived blackout windows,NONE,false,published schedule record,probe_complete,12-structure-outline-bernankey-mvp-cycle.md:278-285
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase2/owned_state_transitions.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/mvp_phase2/owned_state_transitions.csv
size_bytes: 1979
mode_octal: "0644"
modified_at_utc: 2026-09-04T17:58:23.959706Z
sha256: 53377426be02a48d9a312f12c5dcaa47564db296e296776daed89b3a25ec73bc
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
state_id,transition_kind,provenance
state.adapter.market.us.treasury_demand.phase2.boundary,initialize_boundary,12-structure-outline-bernankey-mvp-cycle.md:260-264
state.body.us.federal_reserve.fomc.procedure,record_fomc_decision,12-structure-outline-bernankey-mvp-cycle.md:246-259
state.inst.us.federal_reserve.board.governance,initialize_governance,12-structure-outline-bernankey-mvp-cycle.md:195-205
state.inst.us.federal_reserve.new_york.desk_authority,record_desk_execution,12-structure-outline-bernankey-mvp-cycle.md:260-264
state.legal.us.federal_reserve.act.section_12a.effective_clauses,initialize_effective_clauses,12-structure-outline-bernankey-mvp-cycle.md:195-205
state.legal.us.federal_reserve.act.section_14.effective_clauses,initialize_effective_clauses,12-structure-outline-bernankey-mvp-cycle.md:195-205
state.legal.us.federal_reserve.domestic_authorization.2006.effective_clauses,initialize_effective_clauses,12-structure-outline-bernankey-mvp-cycle.md:195-205
state.legal.us.federal_reserve.fomc_rules.section_3.effective_clauses,initialize_effective_clauses,12-structure-outline-bernankey-mvp-cycle.md:195-205
state.office.us.federal_reserve.chief_of_staff.tenure,initialize_tenure,12-structure-outline-bernankey-mvp-cycle.md:226-232
state.office.us.federal_reserve.governor.mvp_roster,initialize_roster,12-structure-outline-bernankey-mvp-cycle.md:226-232
state.record.us.federal_reserve.policy_package.records,record_policy_package,12-structure-outline-bernankey-mvp-cycle.md:233-245
state.role_holder.fomc.governor_1.cognition,integrate_evidence,12-structure-outline-bernankey-mvp-cycle.md:226-232
state.role_holder.fomc.governor_2.cognition,integrate_evidence,12-structure-outline-bernankey-mvp-cycle.md:226-232
state.schedule.us.federal_reserve.blackout.current,initialize_schedule,12-structure-outline-bernankey-mvp-cycle.md:278-285
state.schedule.us.federal_reserve.fomc.calendar,initialize_schedule,12-structure-outline-bernankey-mvp-cycle.md:278-285
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase2/period_variants.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/mvp_phase2/period_variants.csv
size_bytes: 4583
mode_octal: "0644"
modified_at_utc: 2026-09-04T19:42:58.497320Z
sha256: 5cc9e8a2460066f2a5fbfdc5e6dd6965aef13c1357d42a61c5016f3adcae321f
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
catalog_id,variant_id,effective_period,authority_or_eligibility_ref,uncertainty_notes,provenance
adapter.market.us.treasury_demand.phase2,variant.2006,2006-03-27/2006-03-29,NONE,Temporary adapter applies only to the bounded Phase 2 cycle.,12-structure-outline-bernankey-mvp-cycle.md:260-264
body.us.federal_reserve.fomc,variant.2006,2006-02-01/2006-12-31,legal.us.federal_reserve.fomc_rules.section_3,Bounded MVP roster rather than the complete historical Committee.,12-structure-outline-bernankey-mvp-cycle.md:226-232
inst.us.federal_reserve.board,variant.2006,2006-02-01/2006-12-31,legal.us.federal_reserve.fomc_rules.section_3,The MVP preserves Board identity without assigning FOMC or SOMA powers to it.,12-structure-outline-bernankey-mvp-cycle.md:195-205
inst.us.federal_reserve.new_york,variant.2006,2006-01-31/2007-01-30,legal.us.federal_reserve.domestic_authorization.2006,Only selected Desk effects are represented.,12-structure-outline-bernankey-mvp-cycle.md:260-264
legal.us.federal_reserve.act.section_12a,variant.2006,1935-08-23/2100-01-01,legal.us.federal_reserve.act.section_12a,Only the selected FOMC-direction clause is encoded.,Federal Reserve Act section 12A: https://www.federalreserve.gov/aboutthefed/section12a.htm
legal.us.federal_reserve.act.section_14,variant.2006,1935-08-23/2100-01-01,legal.us.federal_reserve.act.section_14,Only the selected Reserve Bank open-market clause is encoded.,Federal Reserve Act section 14: https://www.federalreserve.gov/aboutthefed/section14.htm
legal.us.federal_reserve.domestic_authorization.2006,variant.2006,2006-01-31/2007-01-30,legal.us.federal_reserve.domestic_authorization.2006,Only the MVP execution clause is encoded.,Federal Reserve 2006 FOMC historical materials: https://www.federalreserve.gov/monetarypolicy/fomchistorical2006.htm
legal.us.federal_reserve.fomc_rules.section_3,variant.2006,2005-01-01/2007-01-01,legal.us.federal_reserve.fomc_rules.section_3,Only the selected voting-procedure clause is encoded.,FOMC Rules of Organization: https://www.federalreserve.gov/monetarypolicy/files/fomc_rulesorganization.pdf
office.us.federal_reserve.chief_of_staff,variant.2006,2006-02-01/2006-12-31,NONE,Individual role-holder identity remains intentionally bounded.,12-structure-outline-bernankey-mvp-cycle.md:226-232
office.us.federal_reserve.governor,variant.2006,2006-02-01/2006-12-31,legal.us.federal_reserve.fomc_rules.section_3,Only two fictional bounded role-holders are initialized.,12-structure-outline-bernankey-mvp-cycle.md:226-232
record.us.federal_reserve.action_result,variant.2006,2006-03-27/2006-03-29,legal.us.federal_reserve.domestic_authorization.2006,Only selected Desk action results are represented.,12-structure-outline-bernankey-mvp-cycle.md:195-285
record.us.federal_reserve.authorization_decision,variant.2006,2006-03-27/2006-03-29,legal.us.federal_reserve.fomc_rules.section_3,Only the selected meeting decision is represented.,12-structure-outline-bernankey-mvp-cycle.md:195-285
record.us.federal_reserve.directive,variant.2006,2006-03-27/2006-03-29,legal.us.federal_reserve.act.section_12a,Only the selected meeting directive is represented.,12-structure-outline-bernankey-mvp-cycle.md:195-285
record.us.federal_reserve.policy_package,variant.2006,2006-03-27/2006-03-29,legal.us.federal_reserve.fomc_rules.section_3,Only three prepared MVP packages and their revision history are represented.,12-structure-outline-bernankey-mvp-cycle.md:233-245
record.us.federal_reserve.stage_receipt,variant.2006,2006-03-27/2006-03-29,NONE,Only policy-chain receipts for the bounded cycle are represented.,12-structure-outline-bernankey-mvp-cycle.md:195-285
role_holder.fomc.governor_1,variant.2006,2006-02-01/2006-12-31,legal.us.federal_reserve.fomc_rules.section_3,Fictional bounded participant rather than a historical identity.,12-structure-outline-bernankey-mvp-cycle.md:226-232
role_holder.fomc.governor_2,variant.2006,2006-02-01/2006-12-31,legal.us.federal_reserve.fomc_rules.section_3,Fictional bounded participant rather than a historical identity.,12-structure-outline-bernankey-mvp-cycle.md:226-232
schedule.us.federal_reserve.blackout,variant.2006,2006-03-21/2006-05-10,schedule.us.federal_reserve.fomc,The bounded cycle derives blackout windows for the decision and next-cycle Morning Book.,12-structure-outline-bernankey-mvp-cycle.md:551-615
schedule.us.federal_reserve.fomc,variant.2006,2006-03-21/2006-05-10,legal.us.federal_reserve.fomc_rules.section_3,The selected occurrences define the opening decision and next-cycle boundary.,12-structure-outline-bernankey-mvp-cycle.md:551-615
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase2/profile_catalog_roles.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/mvp_phase2/profile_catalog_roles.csv
size_bytes: 3042
mode_octal: "0644"
modified_at_utc: 2026-09-04T17:58:23.959960Z
sha256: 4888510776516af56ad467f49e943c213e4959f329209fea4b36cf62aaafa298
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
profile_id,catalog_id,profile_role,candidate_provider_entry_id,activation_requirement,rationale,provenance
profile.early_2006.bernankey,adapter.market.us.treasury_demand.phase2,boundary_candidate,adapter.market.us.treasury_demand.phase2,Selected package reaches a witnessed temporary Treasury-demand boundary.,Phase 2 declares adapter sourcing and defers endogenous market clearing.,12-structure-outline-bernankey-mvp-cycle.md:260-264
profile.early_2006.bernankey,legal.us.federal_reserve.act.section_12a,reference,NONE,Selected FOMC authority clause is effective.,Statutory source for collective FOMC direction.,Federal Reserve Act section 12A: https://www.federalreserve.gov/aboutthefed/section12a.htm
profile.early_2006.bernankey,legal.us.federal_reserve.act.section_14,reference,NONE,Selected Reserve Bank power clause is effective.,Statutory source for Reserve Bank open-market power.,Federal Reserve Act section 14: https://www.federalreserve.gov/aboutthefed/section14.htm
profile.early_2006.bernankey,legal.us.federal_reserve.domestic_authorization.2006,reference,NONE,Selected domestic authorization is effective.,Delegation source for bounded Desk execution.,Federal Reserve 2006 FOMC historical materials: https://www.federalreserve.gov/monetarypolicy/fomchistorical2006.htm
profile.early_2006.bernankey,legal.us.federal_reserve.fomc_rules.section_3,reference,NONE,Selected voting rule is effective.,Procedure source for collective authorization.,FOMC Rules of Organization: https://www.federalreserve.gov/monetarypolicy/files/fomc_rulesorganization.pdf
profile.early_2006.bernankey,record.us.federal_reserve.action_result,reference,NONE,Desk execution path is active.,Preserves execution status and failure stage.,12-structure-outline-bernankey-mvp-cycle.md:195-285
profile.early_2006.bernankey,record.us.federal_reserve.authorization_decision,reference,NONE,FOMC meeting path is active.,Preserves collective authorization separately from proposal and execution.,12-structure-outline-bernankey-mvp-cycle.md:195-285
profile.early_2006.bernankey,record.us.federal_reserve.directive,reference,NONE,Approved or narrowed motion produces a directive.,Carries exact approved effects to the responsible Desk owner.,12-structure-outline-bernankey-mvp-cycle.md:195-285
profile.early_2006.bernankey,record.us.federal_reserve.stage_receipt,reference,NONE,Policy chain emits stage receipts.,Prevents proposal authorization execution settlement and observation conflation.,12-structure-outline-bernankey-mvp-cycle.md:195-285
profile.early_2006.bernankey,role_holder.fomc.governor_1,slice_candidate,role_holder.fomc.governor_1,Bounded 2006 meeting roster is selected.,Supplies one deterministic sourced participant position and vote.,12-structure-outline-bernankey-mvp-cycle.md:226-232
profile.early_2006.bernankey,role_holder.fomc.governor_2,slice_candidate,role_holder.fomc.governor_2,Bounded 2006 meeting roster is selected.,Supplies one deterministic sourced participant position and vote.,12-structure-outline-bernankey-mvp-cycle.md:226-232
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase3/entities.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/mvp_phase3/entities.csv
size_bytes: 661
mode_octal: "0644"
modified_at_utc: 2026-09-04T18:32:34.642441Z
sha256: 0703b0ad112829cf5e475d670aaf4f61bf59a0d301305e92657de3b32a5459b1
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
catalog_id,display_name,entry_class,instance_of,identity_clade,domain,selectable_in_manifest,definition_version,cognition_class,default_fidelity,research_status,completeness_state,fit,fallback_entry_id,residual_counterpart_id,uncertainty_notes,provenance
adapter.market.us.treasury.external_buyer,External Treasury-duration buyer residual,instance,type.boundary_adapter.default,BoundaryAdapter,markets,false,1,none,MECHANICAL_OR_ADAPTER,sketched,probe_complete,fits,UNKNOWN,NONE,Owns only bounded external demand capacity and its cash and Treasury residual accounts; it does not own domestic price formation.,12-structure-outline-bernankey-mvp-cycle.md:299-319
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase3/entity_authority_sources.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/mvp_phase3/entity_authority_sources.csv
size_bytes: 460
mode_octal: "0644"
modified_at_utc: 2026-09-04T18:32:34.642874Z
sha256: eb11917571b3872cac982e9be9333fe168b90516b79417da1ec30c3e8301402f
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
catalog_id,authority_source_id,authority_source_kind,provenance
agreement.us.repo.bilateral,agreement.us.repo.bilateral,contract,12-structure-outline-bernankey-mvp-cycle.md:299-319
cohort.us.dealer.primary,auth.us.primary_dealer_designation,eligibility,12-structure-outline-bernankey-mvp-cycle.md:299-319
market.us.treasury.secondary,legal.us.federal_reserve.domestic_authorization.2006,authorized_operation,12-structure-outline-bernankey-mvp-cycle.md:299-319
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase3/entity_fallback_contracts.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/mvp_phase3/entity_fallback_contracts.csv
size_bytes: 603
mode_octal: "0644"
modified_at_utc: 2026-09-04T18:32:34.642746Z
sha256: 8252d4084bf2683584ab40c813c7082d2bbacfd5e5a32b7c386e5be9083398aa
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
catalog_id,preserves_or_loses,boundary_item,provenance
agreement.us.repo.bilateral,lose,scenario.fallback.FAIL,12-structure-outline-bernankey-mvp-cycle.md:299-319
adapter.market.us.treasury.external_buyer,lose,scenario.fallback.FAIL,12-structure-outline-bernankey-mvp-cycle.md:299-319
cohort.us.dealer.primary,lose,scenario.fallback.FAIL,12-structure-outline-bernankey-mvp-cycle.md:299-319
inst.us.leveraged_funds,lose,scenario.fallback.FAIL,12-structure-outline-bernankey-mvp-cycle.md:299-319
market.us.treasury.secondary,lose,scenario.fallback.FAIL,12-structure-outline-bernankey-mvp-cycle.md:299-319
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase3/owned_state.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/mvp_phase3/owned_state.csv
size_bytes: 4012
mode_octal: "0644"
modified_at_utc: 2026-09-04T18:34:09.097252Z
sha256: d83985127a30aa452a059e01b6da0d14e2f16af402167e46fc0aeb83aebafa45
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
state_id,owner_id,state_kind,value_domain,unit,conserved,witness_kind,completeness_state,provenance
state.agreement.us.repo.bilateral.contract,agreement.us.repo.bilateral,obligation,principal collateral haircut maturity roll decision and status,NONE,false,agreement transition witness,probe_complete,12-structure-outline-bernankey-mvp-cycle.md:299-319
state.adapter.market.us.treasury.external_buyer.cash,adapter.market.us.treasury.external_buyer,stock,nonnegative cash balance,USD,true,double-entry accounting witness,probe_complete,12-structure-outline-bernankey-mvp-cycle.md:299-319
state.adapter.market.us.treasury.external_buyer.demand,adapter.market.us.treasury.external_buyer,capacity,external demand capacity and bid limit,NONE,false,boundary order witness,probe_complete,12-structure-outline-bernankey-mvp-cycle.md:299-319
state.adapter.market.us.treasury.external_buyer.treasury,adapter.market.us.treasury.external_buyer,stock,nonnegative 5-10 year Treasury face,treasury_face,true,double-entry accounting witness,probe_complete,12-structure-outline-bernankey-mvp-cycle.md:299-319
state.cohort.us.dealer.primary.capacity,cohort.us.dealer.primary,capacity,bounded balance-sheet intermediation capacity,NONE,false,dealer order witness,probe_complete,12-structure-outline-bernankey-mvp-cycle.md:299-319
state.cohort.us.dealer.primary.cash,cohort.us.dealer.primary,stock,nonnegative cash balance,USD,true,double-entry accounting witness,probe_complete,12-structure-outline-bernankey-mvp-cycle.md:299-319
state.cohort.us.dealer.primary.collateral_control,cohort.us.dealer.primary,stock,signed collateral control memorandum,treasury_face,true,double-entry accounting witness,probe_complete,12-structure-outline-bernankey-mvp-cycle.md:299-319
state.cohort.us.dealer.primary.repo_claim,cohort.us.dealer.primary,stock,signed bilateral repo claim,USD,true,double-entry accounting witness,probe_complete,12-structure-outline-bernankey-mvp-cycle.md:299-319
state.cohort.us.dealer.primary.treasury,cohort.us.dealer.primary,stock,nonnegative 5-10 year Treasury face,treasury_face,true,double-entry accounting witness,probe_complete,12-structure-outline-bernankey-mvp-cycle.md:299-319
state.inst.us.federal_reserve.new_york.cash,inst.us.federal_reserve.new_york,stock,nonnegative cash balance,USD,true,double-entry accounting witness,probe_complete,12-structure-outline-bernankey-mvp-cycle.md:299-319
state.inst.us.federal_reserve.new_york.treasury,inst.us.federal_reserve.new_york,stock,nonnegative 5-10 year Treasury face,treasury_face,true,double-entry accounting witness,probe_complete,12-structure-outline-bernankey-mvp-cycle.md:299-319
state.inst.us.leveraged_funds.behavior,inst.us.leveraged_funds,condition,liquidity buffer leverage limit and witnessed deficit,NONE,false,cohort order witness,probe_complete,12-structure-outline-bernankey-mvp-cycle.md:299-319
state.inst.us.leveraged_funds.cash,inst.us.leveraged_funds,stock,nonnegative cash balance,USD,true,double-entry accounting witness,probe_complete,12-structure-outline-bernankey-mvp-cycle.md:299-319
state.inst.us.leveraged_funds.collateral_encumbrance,inst.us.leveraged_funds,stock,signed collateral encumbrance memorandum,treasury_face,true,double-entry accounting witness,probe_complete,12-structure-outline-bernankey-mvp-cycle.md:299-319
state.inst.us.leveraged_funds.repo_obligation,inst.us.leveraged_funds,stock,signed bilateral repo obligation,USD,true,double-entry accounting witness,probe_complete,12-structure-outline-bernankey-mvp-cycle.md:299-319
state.inst.us.leveraged_funds.treasury,inst.us.leveraged_funds,stock,nonnegative unencumbered 5-10 year Treasury face,treasury_face,true,double-entry accounting witness,probe_complete,12-structure-outline-bernankey-mvp-cycle.md:299-319
state.market.us.treasury.secondary.clearing,market.us.treasury.secondary,clearing_result,maturity bucket order book price allocation residual and failure,NONE,false,market clearing witness,probe_complete,12-structure-outline-bernankey-mvp-cycle.md:299-319
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase3/owned_state_transitions.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/mvp_phase3/owned_state_transitions.csv
size_bytes: 2189
mode_octal: "0644"
modified_at_utc: 2026-09-04T18:32:34.643270Z
sha256: 4d8ef7d818847a5098556ae22ad151cabe53296734f4b205d45772d5c9ecd20b
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
state_id,transition_kind,provenance
state.agreement.us.repo.bilateral.contract,record_non_roll,12-structure-outline-bernankey-mvp-cycle.md:299-319
state.agreement.us.repo.bilateral.contract,settle_repo,12-structure-outline-bernankey-mvp-cycle.md:299-319
state.adapter.market.us.treasury.external_buyer.cash,accounting_entry,12-structure-outline-bernankey-mvp-cycle.md:299-319
state.adapter.market.us.treasury.external_buyer.demand,submit_order,12-structure-outline-bernankey-mvp-cycle.md:299-319
state.adapter.market.us.treasury.external_buyer.treasury,accounting_entry,12-structure-outline-bernankey-mvp-cycle.md:299-319
state.cohort.us.dealer.primary.capacity,submit_order,12-structure-outline-bernankey-mvp-cycle.md:299-319
state.cohort.us.dealer.primary.cash,accounting_entry,12-structure-outline-bernankey-mvp-cycle.md:299-319
state.cohort.us.dealer.primary.collateral_control,accounting_entry,12-structure-outline-bernankey-mvp-cycle.md:299-319
state.cohort.us.dealer.primary.repo_claim,accounting_entry,12-structure-outline-bernankey-mvp-cycle.md:299-319
state.cohort.us.dealer.primary.treasury,accounting_entry,12-structure-outline-bernankey-mvp-cycle.md:299-319
state.inst.us.federal_reserve.new_york.cash,accounting_entry,12-structure-outline-bernankey-mvp-cycle.md:299-319
state.inst.us.federal_reserve.new_york.treasury,accounting_entry,12-structure-outline-bernankey-mvp-cycle.md:299-319
state.inst.us.leveraged_funds.behavior,record_liquidity_deficit,12-structure-outline-bernankey-mvp-cycle.md:299-319
state.inst.us.leveraged_funds.behavior,submit_order,12-structure-outline-bernankey-mvp-cycle.md:299-319
state.inst.us.leveraged_funds.cash,accounting_entry,12-structure-outline-bernankey-mvp-cycle.md:299-319
state.inst.us.leveraged_funds.collateral_encumbrance,accounting_entry,12-structure-outline-bernankey-mvp-cycle.md:299-319
state.inst.us.leveraged_funds.repo_obligation,accounting_entry,12-structure-outline-bernankey-mvp-cycle.md:299-319
state.inst.us.leveraged_funds.treasury,accounting_entry,12-structure-outline-bernankey-mvp-cycle.md:299-319
state.market.us.treasury.secondary.clearing,record_market_clearing,12-structure-outline-bernankey-mvp-cycle.md:299-319
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase3/period_variants.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/mvp_phase3/period_variants.csv
size_bytes: 1141
mode_octal: "0644"
modified_at_utc: 2026-09-04T18:32:34.642620Z
sha256: 21cd858c4e2ed5dc613ce61819c0ff37f16df454b87be95ece7800ed15d70356
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
catalog_id,variant_id,effective_period,authority_or_eligibility_ref,uncertainty_notes,provenance
agreement.us.repo.bilateral,variant.2006,2006-03-27/2006-03-28,agreement.us.repo.bilateral,One bounded bilateral repo is selected; no automatic renewal is permitted.,12-structure-outline-bernankey-mvp-cycle.md:299-319
adapter.market.us.treasury.external_buyer,variant.2006,2006-03-27/2006-03-29,NONE,External demand is a bounded residual schedule rather than a foreign-sector model.,12-structure-outline-bernankey-mvp-cycle.md:299-319
cohort.us.dealer.primary,variant.2006,2006-03-27/2006-03-29,auth.us.primary_dealer_designation,Aggregate capacity is selected for the bounded cycle.,12-structure-outline-bernankey-mvp-cycle.md:299-319
inst.us.leveraged_funds,variant.2006,2006-03-27/2006-03-29,NONE,One aggregate leveraged-fund cohort is selected for the repo-triggered sale.,12-structure-outline-bernankey-mvp-cycle.md:299-319
market.us.treasury.secondary,variant.2006,2006-03-27/2006-03-29,legal.us.federal_reserve.domestic_authorization.2006,Only the 5-10 year maturity bucket is active.,12-structure-outline-bernankey-mvp-cycle.md:299-319
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase3/profile_catalog_roles.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/mvp_phase3/profile_catalog_roles.csv
size_bytes: 406
mode_octal: "0644"
modified_at_utc: 2026-09-04T18:33:26.192228Z
sha256: 3e017a12d6676069f6a8129e0deedd77581f97a232d2a867effad82d8e42382c
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
profile_id,catalog_id,profile_role,candidate_provider_entry_id,activation_requirement,rationale,provenance
profile.early_2006.bernankey,adapter.market.us.treasury.external_buyer,boundary_candidate,adapter.market.us.treasury.external_buyer,External duration demand residual is required.,Supplies outside demand without owning the domestic clearing price.,12-structure-outline-bernankey-mvp-cycle.md:299-319
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase4/entity_authority_sources.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/mvp_phase4/entity_authority_sources.csv
size_bytes: 775
mode_octal: "0644"
modified_at_utc: 2026-09-04T18:54:05.652447Z
sha256: 138782b4a9ff1b40dec04545c49f004777bb24d861d4f537c1a0c48c3aba7f9f
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
catalog_id,authority_source_id,authority_source_kind,provenance
record.us.federal_reserve.analytical_task,office.us.federal_reserve.board_chair,requesting_office,12-structure-outline-bernankey-mvp-cycle.md:388-457
record.us.federal_reserve.assessment,staff.us.federal_reserve.markets,authoring_staff_unit,12-structure-outline-bernankey-mvp-cycle.md:388-457
staff.us.federal_reserve.communications,inst.us.federal_reserve.board,parent_institution,12-structure-outline-bernankey-mvp-cycle.md:397-415
staff.us.federal_reserve.markets,inst.us.federal_reserve.board,parent_institution,12-structure-outline-bernankey-mvp-cycle.md:397-415
staff.us.federal_reserve.monetary_affairs,inst.us.federal_reserve.board,parent_institution,12-structure-outline-bernankey-mvp-cycle.md:397-415
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase4/entity_fallback_contracts.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/mvp_phase4/entity_fallback_contracts.csv
size_bytes: 649
mode_octal: "0644"
modified_at_utc: 2026-09-04T18:54:05.652594Z
sha256: 59c3b08697cbc338842e7f1698e8676669870a40ba0e2f4cec419a891b626ca0
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
catalog_id,preserves_or_loses,boundary_item,provenance
record.us.federal_reserve.analytical_task,lose,scenario.fallback.FAIL,12-structure-outline-bernankey-mvp-cycle.md:388-457
record.us.federal_reserve.assessment,lose,scenario.fallback.FAIL,12-structure-outline-bernankey-mvp-cycle.md:388-457
staff.us.federal_reserve.communications,lose,scenario.fallback.FAIL,12-structure-outline-bernankey-mvp-cycle.md:388-457
staff.us.federal_reserve.markets,lose,scenario.fallback.FAIL,12-structure-outline-bernankey-mvp-cycle.md:388-457
staff.us.federal_reserve.monetary_affairs,lose,scenario.fallback.FAIL,12-structure-outline-bernankey-mvp-cycle.md:388-457
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase4/owned_state.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/mvp_phase4/owned_state.csv
size_bytes: 1881
mode_octal: "0644"
modified_at_utc: 2026-09-04T18:54:05.652702Z
sha256: 507e08e72259697ec697a93b372e7cd271adda0b2d48a15e9428915257beb9c5
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
state_id,owner_id,state_kind,value_domain,unit,conserved,witness_kind,completeness_state,provenance
state.record.us.federal_reserve.analytical_task.records,record.us.federal_reserve.analytical_task,record,request assignment deadline displacement status and result witness,NONE,false,analytical task lifecycle witness,probe_complete,12-structure-outline-bernankey-mvp-cycle.md:388-457
state.record.us.federal_reserve.assessment.records,record.us.federal_reserve.assessment,record,conclusion distribution evidence assumptions uncertainty alternatives dissent and next information,NONE,false,assessment authorship and delivery witness,probe_complete,12-structure-outline-bernankey-mvp-cycle.md:388-457
state.staff.us.federal_reserve.communications.work,staff.us.federal_reserve.communications,capacity,bounded access methods reservations deliverables and task state,staff_capacity_units,false,staff assignment and capacity witness,probe_complete,12-structure-outline-bernankey-mvp-cycle.md:397-415
state.staff.us.federal_reserve.markets.evidence,staff.us.federal_reserve.markets,record,unit-scoped evidence deliveries with provenance and stale or strategic uncertainty,NONE,false,unit-scoped delivery witness,probe_complete,12-structure-outline-bernankey-mvp-cycle.md:388-457
state.staff.us.federal_reserve.markets.work,staff.us.federal_reserve.markets,capacity,bounded access methods reservations deliverables displacement and task state,staff_capacity_units,false,staff assignment displacement and deadline witness,probe_complete,12-structure-outline-bernankey-mvp-cycle.md:388-457
state.staff.us.federal_reserve.monetary_affairs.work,staff.us.federal_reserve.monetary_affairs,capacity,bounded access methods reservations deliverables and dissent state,staff_capacity_units,false,staff dissent and capacity witness,probe_complete,12-structure-outline-bernankey-mvp-cycle.md:388-457
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase4/owned_state_transitions.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/mvp_phase4/owned_state_transitions.csv
size_bytes: 1955
mode_octal: "0644"
modified_at_utc: 2026-09-04T18:54:05.652822Z
sha256: 7ec54850f283671e2dbb1825cf254404feb735ddad4be7175888bf449c7a2fae
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
state_id,transition_kind,provenance
state.record.us.federal_reserve.analytical_task.records,record_request,12-structure-outline-bernankey-mvp-cycle.md:388-457
state.record.us.federal_reserve.analytical_task.records,record_assignment,12-structure-outline-bernankey-mvp-cycle.md:388-457
state.record.us.federal_reserve.analytical_task.records,record_task_result,12-structure-outline-bernankey-mvp-cycle.md:388-457
state.record.us.federal_reserve.assessment.records,record_assessment,12-structure-outline-bernankey-mvp-cycle.md:388-457
state.record.us.federal_reserve.assessment.records,record_dissent,12-structure-outline-bernankey-mvp-cycle.md:388-457
state.record.us.federal_reserve.assessment.records,record_delivery,12-structure-outline-bernankey-mvp-cycle.md:388-457
state.staff.us.federal_reserve.communications.work,reserve_capacity,12-structure-outline-bernankey-mvp-cycle.md:397-415
state.staff.us.federal_reserve.communications.work,release_capacity,12-structure-outline-bernankey-mvp-cycle.md:397-415
state.staff.us.federal_reserve.markets.evidence,integrate_scoped_delivery,12-structure-outline-bernankey-mvp-cycle.md:388-457
state.staff.us.federal_reserve.markets.work,reserve_capacity,12-structure-outline-bernankey-mvp-cycle.md:388-457
state.staff.us.federal_reserve.markets.work,release_capacity,12-structure-outline-bernankey-mvp-cycle.md:388-457
state.staff.us.federal_reserve.markets.work,displace_work,12-structure-outline-bernankey-mvp-cycle.md:388-457
state.staff.us.federal_reserve.markets.work,record_deadline_result,12-structure-outline-bernankey-mvp-cycle.md:388-457
state.staff.us.federal_reserve.monetary_affairs.work,reserve_capacity,12-structure-outline-bernankey-mvp-cycle.md:388-457
state.staff.us.federal_reserve.monetary_affairs.work,release_capacity,12-structure-outline-bernankey-mvp-cycle.md:388-457
state.staff.us.federal_reserve.monetary_affairs.work,record_dissent,12-structure-outline-bernankey-mvp-cycle.md:388-457
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase4/period_variants.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/mvp_phase4/period_variants.csv
size_bytes: 1311
mode_octal: "0644"
modified_at_utc: 2026-09-04T19:42:58.497235Z
sha256: f88b193e7c7f2e976da0879a603ce74d7035745630dddaabf08239041f84e93e
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
catalog_id,variant_id,effective_period,authority_or_eligibility_ref,uncertainty_notes,provenance
record.us.federal_reserve.analytical_task,variant.2006,2006-03-27/2006-05-10,office.us.federal_reserve.board_chair,The selected request and commitment-monitoring work persist through the next-cycle review.,12-structure-outline-bernankey-mvp-cycle.md:551-615
record.us.federal_reserve.assessment,variant.2006,2006-03-27/2006-05-10,staff.us.federal_reserve.markets,The selected assessment dissent next Morning Book and staff review persist through the cycle boundary.,12-structure-outline-bernankey-mvp-cycle.md:551-615
staff.us.federal_reserve.communications,variant.2006,2006-02-01/2006-12-31,inst.us.federal_reserve.board,Only bounded access methods and capacity are initialized.,12-structure-outline-bernankey-mvp-cycle.md:397-415
staff.us.federal_reserve.markets,variant.2006,2006-02-01/2006-12-31,inst.us.federal_reserve.board,Only dealer-capacity evidence one follow-up and one displaced deliverable are initialized.,12-structure-outline-bernankey-mvp-cycle.md:388-457
staff.us.federal_reserve.monetary_affairs,variant.2006,2006-02-01/2006-12-31,inst.us.federal_reserve.board,Only bounded policy methods capacity and one assessment dissent are initialized.,12-structure-outline-bernankey-mvp-cycle.md:388-457
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase5/entities.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/mvp_phase5/entities.csv
size_bytes: 694
mode_octal: "0644"
modified_at_utc: 2026-09-04T19:14:11.660143Z
sha256: 6632bea5584aa2a9231be61c27379729f9dad4804c7d4b72d8279a53caf09982
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
catalog_id,display_name,entry_class,instance_of,identity_clade,domain,selectable_in_manifest,definition_version,cognition_class,default_fidelity,research_status,completeness_state,fit,fallback_entry_id,residual_counterpart_id,uncertainty_notes,provenance
pop.us.public.low_attention_residual,Low-attention public residual,instance,type.pop_lens.default,PopLens,populations_organizations,false,1,distributed_response,POP_DISTRIBUTED_RESPONSE,sketched,probe_complete,fits,population.us.person.cells,NONE,Scenario-bounded non-owning residual view for direct report delivery; it has no network topology and owns no population or sentiment state.,12-structure-outline-bernankey-mvp-cycle.md:465-543
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase5/entity_authority_sources.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/mvp_phase5/entity_authority_sources.csv
size_bytes: 871
mode_octal: "0644"
modified_at_utc: 2026-09-04T19:14:11.660271Z
sha256: 92c733c311f781380e1b9839e6710199cf7a92777e9ec40578ec05ee5205bb40
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
catalog_id,authority_source_id,authority_source_kind,provenance
household.us.cohorts,profile.early_2006.bernankey,scenario_boundary_contract,12-structure-outline-bernankey-mvp-cycle.md:465-543
outlet.media.loonberg,institution.media.loonberg,operating_institution,12-structure-outline-bernankey-mvp-cycle.md:465-543
pop.us.fixed.rate.homeowners.by.mortgage.vintage,population.us.person.cells,derived_population_view,12-structure-outline-bernankey-mvp-cycle.md:465-543
pop.us.public.low_attention_residual,population.us.person.cells,derived_population_view,12-structure-outline-bernankey-mvp-cycle.md:465-543
pop.us.workers.by.sector,population.us.person.cells,derived_population_view,12-structure-outline-bernankey-mvp-cycle.md:465-543
population.us.person.cells,profile.early_2006.bernankey,scenario_boundary_contract,12-structure-outline-bernankey-mvp-cycle.md:465-543
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase5/entity_fallback_contracts.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/mvp_phase5/entity_fallback_contracts.csv
size_bytes: 716
mode_octal: "0644"
modified_at_utc: 2026-09-04T19:14:11.660411Z
sha256: 99743dc88a94106ad6848dae409b551adf90fd3e9b814a87f41cc6a60c2ec34e
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
catalog_id,preserves_or_loses,boundary_item,provenance
household.us.cohorts,lose,scenario.fallback.FAIL,12-structure-outline-bernankey-mvp-cycle.md:465-543
outlet.media.loonberg,lose,scenario.fallback.FAIL,12-structure-outline-bernankey-mvp-cycle.md:465-543
pop.us.fixed.rate.homeowners.by.mortgage.vintage,lose,scenario.fallback.FAIL,12-structure-outline-bernankey-mvp-cycle.md:465-543
pop.us.public.low_attention_residual,lose,scenario.fallback.FAIL,12-structure-outline-bernankey-mvp-cycle.md:465-543
pop.us.workers.by.sector,lose,scenario.fallback.FAIL,12-structure-outline-bernankey-mvp-cycle.md:465-543
population.us.person.cells,lose,scenario.fallback.FAIL,12-structure-outline-bernankey-mvp-cycle.md:465-543
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase5/owned_state.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/mvp_phase5/owned_state.csv
size_bytes: 893
mode_octal: "0644"
modified_at_utc: 2026-09-04T19:14:11.660533Z
sha256: 03ffb3a1a58cfd577d6cdee81c7f04dd280dbb1fdcb43dac39741a46ee834de4
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
state_id,owner_id,state_kind,value_domain,unit,conserved,witness_kind,completeness_state,provenance
state.household.us.cohorts.allocations,household.us.cohorts,record,two household summaries member allocations tenure and borrowing-cost exposure,person_count,false,household allocation reconciliation witness,probe_complete,12-structure-outline-bernankey-mvp-cycle.md:465-543
state.outlet.media.loonberg.publication,outlet.media.loonberg,publication,editorial selection framing publication queue omissions and direct audience targets,NONE,false,outlet publication witness,probe_complete,12-structure-outline-bernankey-mvp-cycle.md:465-543
state.population.us.person.cells.mass,population.us.person.cells,stock,two scenario-bounded cells with employment and housing exposure,person_count,true,person mass reconciliation witness,probe_complete,12-structure-outline-bernankey-mvp-cycle.md:465-543
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase5/owned_state_transitions.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/mvp_phase5/owned_state_transitions.csv
size_bytes: 391
mode_octal: "0644"
modified_at_utc: 2026-09-04T19:14:11.660651Z
sha256: c1c9eb18fb259135ab2a08acc5dc4236898ecc46e722064faf0c422c5b685322
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
state_id,transition_kind,provenance
state.household.us.cohorts.allocations,initialize_household_allocations,12-structure-outline-bernankey-mvp-cycle.md:465-543
state.outlet.media.loonberg.publication,record_report_publication,12-structure-outline-bernankey-mvp-cycle.md:465-543
state.population.us.person.cells.mass,initialize_person_mass,12-structure-outline-bernankey-mvp-cycle.md:465-543
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase5/period_variants.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/mvp_phase5/period_variants.csv
size_bytes: 1372
mode_octal: "0644"
modified_at_utc: 2026-09-04T19:14:11.660739Z
sha256: 589ccc79a74765f7c9dac9fcb4ad032fa6ba541cf90aff125c501702ae096d91
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
catalog_id,variant_id,effective_period,authority_or_eligibility_ref,uncertainty_notes,provenance
household.us.cohorts,variant.2006,2006-03-27/2006-03-28,profile.early_2006.bernankey,Only two bounded household summaries are selected.,12-structure-outline-bernankey-mvp-cycle.md:465-543
outlet.media.loonberg,variant.2006,2006-03-27/2006-03-28,institution.media.loonberg,Only one FOMC statement and one direct-delivery report are selected.,12-structure-outline-bernankey-mvp-cycle.md:465-543
pop.us.fixed.rate.homeowners.by.mortgage.vintage,variant.2006,2006-03-27/2006-03-28,population.us.person.cells,The lens is a scenario projection and not a national estimate.,12-structure-outline-bernankey-mvp-cycle.md:465-543
pop.us.public.low_attention_residual,variant.2006,2006-03-27/2006-03-28,population.us.person.cells,The residual is a bounded direct audience and not a general media model.,12-structure-outline-bernankey-mvp-cycle.md:465-543
pop.us.workers.by.sector,variant.2006,2006-03-27/2006-03-28,population.us.person.cells,The lens is a scenario projection and not a national estimate.,12-structure-outline-bernankey-mvp-cycle.md:465-543
population.us.person.cells,variant.2006,2006-03-27/2006-03-28,profile.early_2006.bernankey,Two cells preserve only employment and mortgage exposure needed by the selected cycle.,12-structure-outline-bernankey-mvp-cycle.md:465-543
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase5/profile_catalog_roles.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/mvp_phase5/profile_catalog_roles.csv
size_bytes: 340
mode_octal: "0644"
modified_at_utc: 2026-09-04T19:14:11.660848Z
sha256: 5e815a30d87398008d79ae9eb0be9951298ab5a2b8c69f9fdf0a62720e1821de
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
profile_id,catalog_id,profile_role,candidate_provider_entry_id,activation_requirement,rationale,provenance
profile.early_2006.bernankey,pop.us.public.low_attention_residual,slice_candidate,NONE,Select direct report delivery with no network topology.,Phase 5 residual audience projection.,12-structure-outline-bernankey-mvp-cycle.md:465-543
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase6/owned_state.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/mvp_phase6/owned_state.csv
size_bytes: 461
mode_octal: "0644"
modified_at_utc: 2026-09-04T19:42:58.497008Z
sha256: df5611d7ecde9fafada0df6314bcee6b658b710670b30d19f37b8f4a400180ce
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
state_id,owner_id,state_kind,value_domain,unit,conserved,witness_kind,completeness_state,provenance
state.body.us.federal_reserve.fomc.commitments,body.us.federal_reserve.fomc,obligation,policy and communication commitments reservations contingent obligations monitoring links expiry breach and settlement history,institutional_capacity_units,false,commitment lifecycle and reservation witness,probe_complete,12-structure-outline-bernankey-mvp-cycle.md:551-615
````

## Artifact: `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase6/owned_state_transitions.csv`

```yaml
source_task: federal-reserve-chair-crisis-management-simulator
relative_path: catalog/inventory/mvp_phase6/owned_state_transitions.csv
size_bytes: 628
mode_octal: "0644"
modified_at_utc: 2026-09-04T19:42:58.497136Z
sha256: f4d5ce80e9d272436acdaebfb32b31fdfd2d7bd4a59bbdb85bf0afb9ab1c0d95
media_type: text/csv
content_encoding: UTF-8
newline_style: LF
original_ends_with_newline: true
```

````csv
state_id,transition_kind,provenance
state.body.us.federal_reserve.fomc.commitments,activate_commitment,12-structure-outline-bernankey-mvp-cycle.md:551-615
state.body.us.federal_reserve.fomc.commitments,breach_commitment,12-structure-outline-bernankey-mvp-cycle.md:551-615
state.body.us.federal_reserve.fomc.commitments,expire_commitment,12-structure-outline-bernankey-mvp-cycle.md:551-615
state.body.us.federal_reserve.fomc.commitments,initialize_commitments,12-structure-outline-bernankey-mvp-cycle.md:551-615
state.body.us.federal_reserve.fomc.commitments,settle_commitment,12-structure-outline-bernankey-mvp-cycle.md:551-615
````

