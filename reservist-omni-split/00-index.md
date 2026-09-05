# Reservist HumanLayer split-corpus index

- original omnibus: `humanlayer-omni.md`
- original artifact count: 201
- original source payload bytes: 4,257,640
- generated bundle count: 9
- artifact identity: source task + relative path; the source contains two distinct `task.md` artifacts, so a bare relative path is not unique across source tasks.

## Bundles

| Bundle | Conceptual scope | Artifacts | Bundle bytes | Approx. tokens | First artifact | Last artifact |
| --- | --- | ---: | ---: | ---: | --- | --- |
| [`01-foundations-and-research.md`](01-foundations-and-research.md) | Project foundations, research evidence, and working design spines | 8 | 297,252 | ~74,313 | 001 `federal-reserve-chair-crisis-management-simulator/01-research-questions-simulator-foundations.md` | 201 `federal-reserve-chair-crisis-simulator-game/task.md` |
| [`02-simulation-kernel-and-mvp-architecture.md`](02-simulation-kernel-and-mvp-architecture.md) | Minimum simulation kernel, MVP vertical slice, implementation cycle, and post-MVP architecture | 4 | 395,426 | ~98,857 | 004 `federal-reserve-chair-crisis-management-simulator/04-design-discussion-minimum-simulation-kernel.md` | 014 `federal-reserve-chair-crisis-management-simulator/14-design-discussion-game-architecture.md` |
| [`03-representation-ontology-and-composition.md`](03-representation-ontology-and-composition.md) | Representation Bible, Representation Catalog, and Burrow Bank composition probe | 3 | 439,974 | ~109,994 | 005 `federal-reserve-chair-crisis-management-simulator/05-design-discussion-representation-bible.md` | 007 `federal-reserve-chair-crisis-management-simulator/07-design-discussion-burrow-composition-probe.md` |
| [`04-catalog-core-ontology-and-validation.md`](04-catalog-core-ontology-and-validation.md) | Catalog implementation, schema, core ontology, state, relationships, profiles, and validation artifacts | 20 | 694,640 | ~173,660 | 015 `federal-reserve-chair-crisis-management-simulator/catalog/__pycache__/catalog.cpython-314.pyc` | 198 `federal-reserve-chair-crisis-management-simulator/catalog/world_profiles.csv` |
| [`05-catalog-mechanisms-and-external-interfaces.md`](05-catalog-mechanisms-and-external-interfaces.md) | Catalog composition probes, instruments, external interfaces, observations, products, research backlog, and transmissions | 24 | 425,417 | ~106,355 | 019 `federal-reserve-chair-crisis-management-simulator/catalog/composition_probe_instrument_buckets.csv` | 195 `federal-reserve-chair-crisis-management-simulator/catalog/transmissions.csv` |
| [`06-generated-planning-and-research-status.md`](06-generated-planning-and-research-status.md) | Generated catalog eligibility, backlog, planning, coverage, gap, and research-status views | 12 | 686,247 | ~171,562 | 030 `federal-reserve-chair-crisis-management-simulator/catalog/generated/catalog_eligibility.csv` | 082 `federal-reserve-chair-crisis-management-simulator/catalog/generated/scope_market_coverage.csv` |
| [`07-generated-domain-and-identity-views.md`](07-generated-domain-and-identity-views.md) | Generated domain and identity-clade catalog views | 41 | 463,474 | ~115,869 | 034 `federal-reserve-chair-crisis-management-simulator/catalog/generated/domain/external_boundary.csv` | 076 `federal-reserve-chair-crisis-management-simulator/catalog/generated/identity_clade/statefulexternalprocess.csv` |
| [`08-inventory-closure-and-mvp-slices.md`](08-inventory-closure-and-mvp-slices.md) | Closure inventory, Federal Reserve/Treasury inventory, and MVP phase inventory slices | 64 | 588,641 | ~147,161 | 088 `federal-reserve-chair-crisis-management-simulator/catalog/inventory/closure/action_domains.csv` | 157 `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase6/owned_state_transitions.csv` |
| [`09-world-inventories-media-and-interface.md`](09-world-inventories-media-and-interface.md) | Information and interface design with geographic, institutional, product, population, and external-world inventories | 25 | 407,511 | ~101,878 | 009 `federal-reserve-chair-crisis-management-simulator/09-design-discussion-economist-pundit-media.md` | 172 `federal-reserve-chair-crisis-management-simulator/catalog/inventory/sovereign_regional/transmission_probes.csv` |

## Suggested model-ingestion order

Read the bundles in numeric order. Start with the foundations and current design spines, then the runtime and representation decisions, then the catalog source data, generated views, and inventory snapshots. This order keeps later integrative architecture adjacent to its research and retains the complete historical record for comparison.

1. `01-foundations-and-research.md`
2. `02-simulation-kernel-and-mvp-architecture.md`
3. `03-representation-ontology-and-composition.md`
4. `04-catalog-core-ontology-and-validation.md`
5. `05-catalog-mechanisms-and-external-interfaces.md`
6. `06-generated-planning-and-research-status.md`
7. `07-generated-domain-and-identity-views.md`
8. `08-inventory-closure-and-mvp-slices.md`
9. `09-world-inventories-media-and-interface.md`

## Complete artifact-to-bundle mapping

| Source order | Artifact identity | Bundle | SHA-256 |
| ---: | --- | --- | --- |
| 001 | `federal-reserve-chair-crisis-management-simulator/01-research-questions-simulator-foundations.md` | `01-foundations-and-research.md` | `494730e19116fb5e94e6f2c9583380448e9a154b288816728a32bd986ea5ce0f` |
| 002 | `federal-reserve-chair-crisis-management-simulator/02-research-clowder-simulation-substrate.md` | `01-foundations-and-research.md` | `cad754a4bd91964914cce7a6b4e67e117a73813f9523a7d7d48c4b7a38cd45f6` |
| 003 | `federal-reserve-chair-crisis-management-simulator/03-research-comparative-simulation-games.md` | `01-foundations-and-research.md` | `0da3cf036b032c230731d1578cc05b84cc7d8a891bed8178081dbe07ec118828` |
| 004 | `federal-reserve-chair-crisis-management-simulator/04-design-discussion-minimum-simulation-kernel.md` | `02-simulation-kernel-and-mvp-architecture.md` | `59819e7dc06f75ff8e2932d1e78bc834b46594746f2243b02f67731b8f93c90b` |
| 005 | `federal-reserve-chair-crisis-management-simulator/05-design-discussion-representation-bible.md` | `03-representation-ontology-and-composition.md` | `ecef48fbe338327099a833ce2bd1ee9a022e8217d7c9d2b3a5580cd4b06fcd07` |
| 006 | `federal-reserve-chair-crisis-management-simulator/06-design-discussion-representation-catalog.md` | `03-representation-ontology-and-composition.md` | `8e8e4ca76e119c7cd172fbe450e5c436a83cdfa8839af5259da0904ce883723a` |
| 007 | `federal-reserve-chair-crisis-management-simulator/07-design-discussion-burrow-composition-probe.md` | `03-representation-ontology-and-composition.md` | `624940e64cff550cb6ed1c411db29767bc614b5f5ecf0a7074db1eb365b51371` |
| 008 | `federal-reserve-chair-crisis-management-simulator/08-research-simulator-foundations.md` | `01-foundations-and-research.md` | `2c39dbd35599763a5ffdcb45c774767b0b24175c1156216eacf644e66db13ea6` |
| 009 | `federal-reserve-chair-crisis-management-simulator/09-design-discussion-economist-pundit-media.md` | `09-world-inventories-media-and-interface.md` | `d1c28738ca627b2b654d51b16bdd880b65c5e9a8c82c65c196bede1077e7a032` |
| 010 | `federal-reserve-chair-crisis-management-simulator/10-design-discussion-epistemic-fairness-interface.md` | `09-world-inventories-media-and-interface.md` | `83efa180af3b9029513db6f79f325d613928504bbe545d72ba424c39e9d0b4b6` |
| 011 | `federal-reserve-chair-crisis-management-simulator/11-design-discussion-bernankey-mvp-slice.md` | `02-simulation-kernel-and-mvp-architecture.md` | `5fd247268b58ecf1667e1130e6b90fa82931bcc02f0c7313caad1e2750c45480` |
| 012 | `federal-reserve-chair-crisis-management-simulator/12-structure-outline-bernankey-mvp-cycle.md` | `02-simulation-kernel-and-mvp-architecture.md` | `c35148474a16a5410398c2fad222f78e191f6dd8e8467d10d3e8070fc65df021` |
| 013 | `federal-reserve-chair-crisis-management-simulator/13-research-game-architecture.md` | `01-foundations-and-research.md` | `a6f333e545991237ea7e4a851f925650115c4c1134b4c72e0c4281f0ed00edcd` |
| 014 | `federal-reserve-chair-crisis-management-simulator/14-design-discussion-game-architecture.md` | `02-simulation-kernel-and-mvp-architecture.md` | `0bbd99f5622302f7ebd1e2dc2724624bddcda714acba2ab7abb355ac13efa70b` |
| 015 | `federal-reserve-chair-crisis-management-simulator/catalog/__pycache__/catalog.cpython-314.pyc` | `04-catalog-core-ontology-and-validation.md` | `3218be73e702aabdb4f3c270c2be82bece64fe6c5f7d0fa8ec4005c64fef25f9` |
| 016 | `federal-reserve-chair-crisis-management-simulator/catalog/__pycache__/test_catalog.cpython-314.pyc` | `04-catalog-core-ontology-and-validation.md` | `99defad82bd630ced75eec412c8467b7d8d10ceb5279b88b92d741693b09c43c` |
| 017 | `federal-reserve-chair-crisis-management-simulator/catalog/action_domains.csv` | `04-catalog-core-ontology-and-validation.md` | `f50072d98f4f91a410fafe31bd0cdbed351da9eba72c3c238e7f0ec4d6d5d119` |
| 018 | `federal-reserve-chair-crisis-management-simulator/catalog/catalog.py` | `04-catalog-core-ontology-and-validation.md` | `34bfab4a8238b7f4ac48dfe782213834432d9da3141cf1f1c2a67f5c48f0f111` |
| 019 | `federal-reserve-chair-crisis-management-simulator/catalog/composition_probe_instrument_buckets.csv` | `05-catalog-mechanisms-and-external-interfaces.md` | `a71099cfb1e8e3386437f7b7507ce310d1b4fa95eb7168e34f896089e1a69437` |
| 020 | `federal-reserve-chair-crisis-management-simulator/catalog/composition_probe_members.csv` | `05-catalog-mechanisms-and-external-interfaces.md` | `deb04c8f332c5b17a717a6ed57083607d161aa69588077b919df0b364a00ed3c` |
| 021 | `federal-reserve-chair-crisis-management-simulator/catalog/composition_probe_requirements.csv` | `05-catalog-mechanisms-and-external-interfaces.md` | `9a00b2201741ae3054a4079acc78f3d8a5b853d54a012a6183609d42b569c4de` |
| 022 | `federal-reserve-chair-crisis-management-simulator/catalog/composition_probes.csv` | `05-catalog-mechanisms-and-external-interfaces.md` | `ff4d9d0b318e8909851279aef5751bfbb74b71b2f55cf6a68f2353efc3d3de1d` |
| 023 | `federal-reserve-chair-crisis-management-simulator/catalog/entities.csv` | `04-catalog-core-ontology-and-validation.md` | `2c8ba8984eea13b6f94bc239d2f5d9185441b441af4f127ed31187cbad6cd35a` |
| 024 | `federal-reserve-chair-crisis-management-simulator/catalog/entity_authority_sources.csv` | `04-catalog-core-ontology-and-validation.md` | `dce01d0daa6f13afb81390ee953e65ad33d4bc682b7705d11f0712bc67cf3731` |
| 025 | `federal-reserve-chair-crisis-management-simulator/catalog/entity_fallback_contracts.csv` | `04-catalog-core-ontology-and-validation.md` | `0fc786f5fd969cf1e8d11f19d88d4e3f36053b68cbead2db591290a02f7be6e3` |
| 026 | `federal-reserve-chair-crisis-management-simulator/catalog/entity_scopes.csv` | `04-catalog-core-ontology-and-validation.md` | `9b3216c80d7499477faf8d9b21211d2c97bbae01145f24f940d0c853313be288` |
| 027 | `federal-reserve-chair-crisis-management-simulator/catalog/external_channel_providers.csv` | `05-catalog-mechanisms-and-external-interfaces.md` | `f54f3175b6e8efffb8c663528459e958cf37ed8bbd359ab7f647c1cec5cbe621` |
| 028 | `federal-reserve-chair-crisis-management-simulator/catalog/external_channels.csv` | `05-catalog-mechanisms-and-external-interfaces.md` | `340a56561c04fda7df40a29cba01a624502038c87653a1ee4201b90126df3c22` |
| 029 | `federal-reserve-chair-crisis-management-simulator/catalog/external_market_channels.csv` | `05-catalog-mechanisms-and-external-interfaces.md` | `3fdd83d2e33338467f8bb31ba04d35fa91991508e60412ff31bbd7dfef49c175` |
| 030 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/catalog_eligibility.csv` | `06-generated-planning-and-research-status.md` | `fcc45bf7db6dd3604293605c49a7f13e45ac8bc782aef4fa335b8ce70fe44160` |
| 031 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/completeness.txt` | `06-generated-planning-and-research-status.md` | `5f1396d001421faa6cfece7a8a694f27c5e6f8fe144f3c949a4016ab372edd59` |
| 032 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/composition_probe_coverage.csv` | `06-generated-planning-and-research-status.md` | `67af19d25ae6c67efb0f57d657cf2f261015fefa58bf2f006d5ae9e54c7492c9` |
| 033 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/deferred_backlog.csv` | `06-generated-planning-and-research-status.md` | `4145f6ae2cbc60273dec8732a9f28f1809c3038fdaaaee3990b5da5479fe3b0b` |
| 034 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/domain/external_boundary.csv` | `07-generated-domain-and-identity-views.md` | `041b3b1163f30a181c9de16ec37aae95ea9c93c555c620eaae0f211b91cc7921` |
| 035 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/domain/external_scope.csv` | `07-generated-domain-and-identity-views.md` | `9a1af891c04015ea4f8182de1403593d533c8aeb7e7f2e4cb463224c642d49d7` |
| 036 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/domain/fed.csv` | `07-generated-domain-and-identity-views.md` | `2e106c4c35c1529f65c13ed6b41930efad1ae637d689aa219003122b169ff516` |
| 037 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/domain/fed_treasury.csv` | `07-generated-domain-and-identity-views.md` | `2cee5268ab160b1c906d2858dabaf1104cac8c328899c0b86365e9d271d112e4` |
| 038 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/domain/financial.csv` | `07-generated-domain-and-identity-views.md` | `cd62025665a1006c9e2224056281ed9aac96ac01756295d4875977bedc4efe6b` |
| 039 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/domain/legal_records.csv` | `07-generated-domain-and-identity-views.md` | `2935e9f990b09494f3a94bd7f232d5f237e2e9f2b365d4eae3518e77b8478c05` |
| 040 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/domain/macro_boundary.csv` | `07-generated-domain-and-identity-views.md` | `35ea26b64d8cab8ba57c6fbdec70a290e785557f14a72b96f2f537c712156f66` |
| 041 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/domain/markets.csv` | `07-generated-domain-and-identity-views.md` | `2e4ed820a00c813c3cc8b36667fb234426e065ebc9dedec07743419610581145` |
| 042 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/domain/media_information.csv` | `07-generated-domain-and-identity-views.md` | `4d75eee3bd24658092e32f0037820e294d4ce1cd60b7677a3f5f10a02d7bb17f` |
| 043 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/domain/physical_products.csv` | `07-generated-domain-and-identity-views.md` | `c7e24547fd8f77e5a16ff526895acc52ce29061c717907d247a1dd295a06f68e` |
| 044 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/domain/populations_organizations.csv` | `07-generated-domain-and-identity-views.md` | `f4ed62ffe03ae584d9f3e4ccbffd34851365b06b284f9fcc864f2d8991ebc485` |
| 045 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/domain/sovereign_regional.csv` | `07-generated-domain-and-identity-views.md` | `823ebe43efc0a40646f32d1ea0808f130abd87f4b5f66c3cf1005006b7f52a1b` |
| 046 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/domain/treasury.csv` | `07-generated-domain-and-identity-views.md` | `47102ecae51c91279e309ca6e9384ae197c6427eedfbdddd0f285f1a24154cf5` |
| 047 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/external_channel_coverage.csv` | `06-generated-planning-and-research-status.md` | `60771cc5e48f5e2be72b7181645940b6e4a5a95281321dea6f27415c05018c0d` |
| 048 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/gaps.csv` | `06-generated-planning-and-research-status.md` | `9a7f006dc1b4b74ac427c90d59509b2a2f0736a7d5a691cf467cb183ad1e3887` |
| 049 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/identity_clade/agreement.csv` | `07-generated-domain-and-identity-views.md` | `6d29345564f5033155b64ff04f75e1749ac4efe8065fa4efbcc7cfa28574f2de` |
| 050 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/identity_clade/boundaryadapter.csv` | `07-generated-domain-and-identity-views.md` | `2c7d1187a064b05386195d1ed708ea073037209ff65a76e1118bceba3cae9c94` |
| 051 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/identity_clade/coalition.csv` | `07-generated-domain-and-identity-views.md` | `6f6135ea35d75bf001438cb9736365b8b2ed1320ef146970667b4d2919b062be` |
| 052 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/identity_clade/decisionbody.csv` | `07-generated-domain-and-identity-views.md` | `1d6522e0601f433eaefa74254cf559645a0b00779a9d59767f20aee4292270b0` |
| 053 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/identity_clade/facility.csv` | `07-generated-domain-and-identity-views.md` | `5474bc5d6635c8f4197e9c2e4a8b2914dff16e1ecfed5d7f42056bc6ca2e1d6c` |
| 054 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/identity_clade/federatedsystem.csv` | `07-generated-domain-and-identity-views.md` | `0e40e628122d146cf9f2e341e7961bf082f6a4da4dff7fbbc9692e53212aa026` |
| 055 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/identity_clade/generator.csv` | `07-generated-domain-and-identity-views.md` | `b906ddb2fc9feb1f6731014a27c5891549eb5eec74c9ccb76fb62b492f66b1e7` |
| 056 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/identity_clade/householdcohort.csv` | `07-generated-domain-and-identity-views.md` | `ff325d5078764a3a53c40b7dd3b3caaac25403100a77427fd8f3e916b4e9a8e0` |
| 057 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/identity_clade/industrycohort.csv` | `07-generated-domain-and-identity-views.md` | `ee3ed649e94b5d6c7783d37496856b72563cf3d2424614e810b8b4eabe741ff6` |
| 058 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/identity_clade/institution.csv` | `07-generated-domain-and-identity-views.md` | `a54d70239e5ee12c57dce07153d5dfd44e0359ed1dfa093bece66b7e1b1b9daf` |
| 059 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/identity_clade/legalinstrument.csv` | `07-generated-domain-and-identity-views.md` | `9dfebb397474563c4fba74c196a5ed91a079546a588da3e9edc6937b57049fe9` |
| 060 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/identity_clade/market.csv` | `07-generated-domain-and-identity-views.md` | `a123132135b69c1ad559eba0cc94e521ec0eed2327c034b0b46e98980e12bff9` |
| 061 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/identity_clade/mechanicalsystem.csv` | `07-generated-domain-and-identity-views.md` | `b373dd2cae76eb7eba5e1b565293b2a82fefcac37895a61e1526a64d3e338bd5` |
| 062 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/identity_clade/namedfirm.csv` | `07-generated-domain-and-identity-views.md` | `5ddc4a1fa0964a1a9abd8b4d24691c63dd5b2354d835ca20a1bd93d9995f2319` |
| 063 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/identity_clade/network.csv` | `07-generated-domain-and-identity-views.md` | `adea5a5ccc3a6026b0358ecdd8e4f1d1a71ec2aa0f74a2c6028b082120bf71aa` |
| 064 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/identity_clade/office.csv` | `07-generated-domain-and-identity-views.md` | `0e9b00a6a84b1d7eaf3a637f92a9a036c7a3f4937799adf0b69dab68693d76cb` |
| 065 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/identity_clade/organizationcohort.csv` | `07-generated-domain-and-identity-views.md` | `b0f88e0534ed30f917013643c16e098572b5568842af484b9cc4691013258ad2` |
| 066 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/identity_clade/outlet.csv` | `07-generated-domain-and-identity-views.md` | `fe35a0322afd32dab1b4ff07bd76cf79d942800fa4b9d849bb619e903394bc1e` |
| 067 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/identity_clade/person.csv` | `07-generated-domain-and-identity-views.md` | `e0ea2db59b6825d6c0eba91c993b5e7766c89fa789a2e37dac6f56392966519e` |
| 068 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/identity_clade/personpopulationcell.csv` | `07-generated-domain-and-identity-views.md` | `b07a50b6dab5c23631b139832bfd313aa7ffc119cd4aa7d6b64e3af053d95d37` |
| 069 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/identity_clade/poplens.csv` | `07-generated-domain-and-identity-views.md` | `507db4e9324f4b892ff11977ca785c63fdb75ddd8737728d92ee58a03a535715` |
| 070 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/identity_clade/publishedreference.csv` | `07-generated-domain-and-identity-views.md` | `b531deafc422d06f715277ece1157575f1b7b6b8f5cf8b578fe6c389d05f4a19` |
| 071 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/identity_clade/record.csv` | `07-generated-domain-and-identity-views.md` | `8b78c71fc9069ab2bdfb4a0f36e4dd660d44b186e1578295e630d6d58c333852` |
| 072 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/identity_clade/region.csv` | `07-generated-domain-and-identity-views.md` | `ffd8db5a139d89f23f95b783323e5e9e2f03c48be67af5371c237505d326b7fd` |
| 073 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/identity_clade/scheduledprocess.csv` | `07-generated-domain-and-identity-views.md` | `1c95347713ef4e0d26393624acbefb05b0d9e709933d94019bfb62ec5d31bebf` |
| 074 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/identity_clade/sovereignsystem.csv` | `07-generated-domain-and-identity-views.md` | `6d37f10a55144e1f877af09360e07a8fad6ce480e7568fd197c092aa6ac29e18` |
| 075 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/identity_clade/staffunit.csv` | `07-generated-domain-and-identity-views.md` | `2422ad87c10d1057e2cd41c2fe95a6ed934f0a6e506d9a4b90b192084b2eb2e1` |
| 076 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/identity_clade/statefulexternalprocess.csv` | `07-generated-domain-and-identity-views.md` | `73d7cbf698940088ea6aa26d1f34918e43b2f81339451cc0c1fdc28981fbec44` |
| 077 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/profile_planning.csv` | `06-generated-planning-and-research-status.md` | `212481cfda2b127220ec9c47f501d2acb500a8d2adfaa43a1aa2c28185c57e0e` |
| 078 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/research_status/named.csv` | `06-generated-planning-and-research-status.md` | `2ab9ef429368e82292a9b27bd8b45edf8b83c570c9673716c58f5157c2e00e30` |
| 079 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/research_status/researched.csv` | `06-generated-planning-and-research-status.md` | `fbff25384e7a67491b15b9a5ade9d8745a3d6368083a07b9fec31ccf433285f6` |
| 080 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/research_status/sketched.csv` | `06-generated-planning-and-research-status.md` | `4ff0ed839c691c2f878897b89a39a92234c2f5cec8f92f41338d38787a818e40` |
| 081 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/research_status/typed.csv` | `06-generated-planning-and-research-status.md` | `bde5fc20302ff7fc50a8128c397d0ba36766f3eea5edce168f776fb47c3c5132` |
| 082 | `federal-reserve-chair-crisis-management-simulator/catalog/generated/scope_market_coverage.csv` | `06-generated-planning-and-research-status.md` | `bc85d0ece1db5f17c97feb5f8508fc8fcf4a9e9386e2e96051576410b3f8f763` |
| 083 | `federal-reserve-chair-crisis-management-simulator/catalog/instrument_bucket_dimensions.csv` | `05-catalog-mechanisms-and-external-interfaces.md` | `f965ba0e1530283eb7ec702d02fa12b86b73295156452d38086d7d43a4bd6fbe` |
| 084 | `federal-reserve-chair-crisis-management-simulator/catalog/instrument_buckets.csv` | `05-catalog-mechanisms-and-external-interfaces.md` | `87ad4f562bc4fae670bfceff09b303454927d753e34e3922eda77dad7e916e4e` |
| 085 | `federal-reserve-chair-crisis-management-simulator/catalog/instrument_families.csv` | `05-catalog-mechanisms-and-external-interfaces.md` | `968de51ec92e22242a7a1c05918093d2834bcd9421fa870f0e08e754eb3a451e` |
| 086 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/arabian_peninsula/entities.csv` | `09-world-inventories-media-and-interface.md` | `e83de2b22e04196a630ac35094f6a43c298b7e0efc39c2e6bf261b6d198147a6` |
| 087 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/caucasus/entities.csv` | `09-world-inventories-media-and-interface.md` | `a9dc4d6c14015d09e08ba7c97cee06c2f5e683f775ee98ce867d26eb54e3ed37` |
| 088 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/closure/action_domains.csv` | `08-inventory-closure-and-mvp-slices.md` | `58659ae1aa6b2c42db70b4b345f09bbf83bd18316a913b47d9c87ae0b7353a10` |
| 089 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/closure/composition_probe_members.csv` | `08-inventory-closure-and-mvp-slices.md` | `40e4358a944a3b83f3191c125afabd7f88dda2bc0761e95a700a3a99b6d88f02` |
| 090 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/closure/composition_probe_requirements.csv` | `08-inventory-closure-and-mvp-slices.md` | `43d9db807199e9e32c6f2dd06e87b190aa056e2093c71f39177f7cdb76cfd1e9` |
| 091 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/closure/composition_probes.csv` | `08-inventory-closure-and-mvp-slices.md` | `bc1821070d89c876335dacbced864ab6e0dda1a3d80b378d579436154bd95396` |
| 092 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/closure/entities.csv` | `08-inventory-closure-and-mvp-slices.md` | `035bce10fc5480d78e26605c0e218ba8852f2d2b640d8e00f75f5aa7341d34de` |
| 093 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/closure/entity_scopes.csv` | `08-inventory-closure-and-mvp-slices.md` | `9b3216c80d7499477faf8d9b21211d2c97bbae01145f24f940d0c853313be288` |
| 094 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/closure/owned_state.csv` | `08-inventory-closure-and-mvp-slices.md` | `90e75c272fabeab540b20081f6148b055bede6d832d4f56b98241ac3764c5c55` |
| 095 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/closure/owned_state_transitions.csv` | `08-inventory-closure-and-mvp-slices.md` | `0c4526cdce3f5aa1eb1115212d0bb26eb2f56aa115db0685b8e3eb99f9c859fa` |
| 096 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/closure/period_variants.csv` | `08-inventory-closure-and-mvp-slices.md` | `873f7e67a5057edb4882c201cc7f42d5d05bb0f4b54b08d741c697d39ad3d94c` |
| 097 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/closure/probe_coverage.csv` | `08-inventory-closure-and-mvp-slices.md` | `ad9073a3b72f5ef0cf77d6cbc911fa0e83ca6bbd8460383657ca9b5f59c27cd4` |
| 098 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/closure/profile_catalog_roles.csv` | `08-inventory-closure-and-mvp-slices.md` | `ce46ab11eaedebc49244f0e6212756e1538a5e0eb8c627380498ed15066dd002` |
| 099 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/closure/relationships.csv` | `08-inventory-closure-and-mvp-slices.md` | `511f017d3d313a0f176ba0c6e278db2dec08e9771a5c5a65a1efb2f1dec454a6` |
| 100 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/closure/research_backlog.csv` | `08-inventory-closure-and-mvp-slices.md` | `cef339a3b25243d5eed41cc027f35b43de4de352be0c9ad9e1cfb8ff4502657a` |
| 101 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/closure/scenario_availability.csv` | `08-inventory-closure-and-mvp-slices.md` | `c82bfdda600a910397fbfc7553a266b960acc0de9727f217aa9e6f9279a7078a` |
| 102 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/closure/scenario_candidates.csv` | `08-inventory-closure-and-mvp-slices.md` | `66c653ad1fa4db9500daf78f765d5c5d1ce09c1784f2851c288e4a32c393a3b4` |
| 103 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/closure/transmission_scenarios.csv` | `08-inventory-closure-and-mvp-slices.md` | `bc895c16c7a777914529b24bcde07b910f00719b2396264f0755fb4e45b2fdcc` |
| 104 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/closure/transmissions.csv` | `08-inventory-closure-and-mvp-slices.md` | `8a2c972e96d22217ec406d86990cce7775adc6b15657dcec5e041a031f8427a7` |
| 105 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/closure/type_fidelity.csv` | `08-inventory-closure-and-mvp-slices.md` | `56dcf9ced7e834025db4bde60f8ee64f18c2bf959130217ef5998e0f0495c5dc` |
| 106 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/closure/types.csv` | `08-inventory-closure-and-mvp-slices.md` | `81c8936447febfeddd54a51d7037ff9fe824881bffc6e280a511cf92e8b3d5a5` |
| 107 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/closure/world_profiles.csv` | `08-inventory-closure-and-mvp-slices.md` | `51863bb6ec6061278abbcd44adf932e6de2c9084111ba980f3b0876a9c82c80f` |
| 108 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/congo/entities.csv` | `09-world-inventories-media-and-interface.md` | `e01c256a3ff9c8892c840c3ce637aeefe49ac07f28da3d0eee0c32130a674389` |
| 109 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/fed_treasury/action_domains.csv` | `08-inventory-closure-and-mvp-slices.md` | `f50072d98f4f91a410fafe31bd0cdbed351da9eba72c3c238e7f0ec4d6d5d119` |
| 110 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/fed_treasury/composition_probe_instrument_buckets.csv` | `08-inventory-closure-and-mvp-slices.md` | `cb7e9f20b606e38f7b68310d885b710cbaeb39250df3f785eb336df510f0104c` |
| 111 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/fed_treasury/entities.csv` | `08-inventory-closure-and-mvp-slices.md` | `9e1f215c20063e59bffd6952f1d9e45e7277972d78efaac983bbcfcb2ecdd338` |
| 112 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/fed_treasury/entity_authority_sources.csv` | `08-inventory-closure-and-mvp-slices.md` | `59083eff44a82a796bb8e9a1c08d19c8d2601a0c30d140617fdcc60c44a522a2` |
| 113 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/fed_treasury/instrument_bucket_dimensions.csv` | `08-inventory-closure-and-mvp-slices.md` | `67a95d3d4c2a26212ea3aba682eec4cb52026f26fea9a2b74eb8c24c8279a020` |
| 114 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/fed_treasury/instrument_buckets.csv` | `08-inventory-closure-and-mvp-slices.md` | `c167fce19cb230a8a996cef85bab974f883debadcc40f44aefadd11dd6e41963` |
| 115 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/fed_treasury/instrument_families.csv` | `08-inventory-closure-and-mvp-slices.md` | `1f1ecd59374958241a09ff33f750e2a1c27183a55b41e3d6644af834d2746dea` |
| 116 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/fed_treasury/presentation_refs.csv` | `08-inventory-closure-and-mvp-slices.md` | `80330b9fbc6bc0cb74ab8cfbc5fba569fddb23d29e3abfd9d799da1a2dfb54ac` |
| 117 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/fed_treasury/profile_required_offices.csv` | `08-inventory-closure-and-mvp-slices.md` | `167347027482ab01b4e30f26bbaa1f27b74f0f597f66843a024fcd7027015426` |
| 118 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/financial/entities.csv` | `09-world-inventories-media-and-interface.md` | `cd62025665a1006c9e2224056281ed9aac96ac01756295d4875977bedc4efe6b` |
| 119 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/legal_records/entities.csv` | `09-world-inventories-media-and-interface.md` | `2935e9f990b09494f3a94bd7f232d5f237e2e9f2b365d4eae3518e77b8478c05` |
| 120 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/media_information/entities.csv` | `09-world-inventories-media-and-interface.md` | `d92fa7612d8823ead8bc151b663b78b7a4d693c60ddb7f228c2513150f43d4f2` |
| 121 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/media_information/relationships.csv` | `09-world-inventories-media-and-interface.md` | `af8f88a2fa1c64f2c1045a1f6c7b8a318aae5bc9329ac82933875303f3c1e584` |
| 122 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/media_information/transmissions.csv` | `09-world-inventories-media-and-interface.md` | `d3c9ae6f67897389b7c9314d8f70d613afed25d6fa6c2b1df8c30850c2bcad21` |
| 123 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase1/entities.csv` | `08-inventory-closure-and-mvp-slices.md` | `35ea26b64d8cab8ba57c6fbdec70a290e785557f14a72b96f2f537c712156f66` |
| 124 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase1/entity_authority_sources.csv` | `08-inventory-closure-and-mvp-slices.md` | `3956369270793ef93ef4c6a79b959702d0a815bac6c83b1bfd3d4a4e0714e247` |
| 125 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase1/entity_fallback_contracts.csv` | `08-inventory-closure-and-mvp-slices.md` | `b027872581379e05da3dcc1d03164687da2805c2d65c46c0e9b14493f302e1fa` |
| 126 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase1/owned_state.csv` | `08-inventory-closure-and-mvp-slices.md` | `2472c5bd5072f4047010a1f95e831eafbd5ae5a2e14e602cd5c5344f8e751cf7` |
| 127 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase1/owned_state_transitions.csv` | `08-inventory-closure-and-mvp-slices.md` | `44c8372f4872e1021f0e7a01c3c6d12ef47d0f717a5d43dedcd1a48db59cceab` |
| 128 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase1/period_variants.csv` | `08-inventory-closure-and-mvp-slices.md` | `6589d0ba1c513e11dd6936a1c28af3ed36de7b5331d6276223a1b9a79b813169` |
| 129 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase1/profile_catalog_roles.csv` | `08-inventory-closure-and-mvp-slices.md` | `9f03fd4e3933601f30b51f67923800453fb6800194a50514560931eb30eb8827` |
| 130 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase2/entities.csv` | `08-inventory-closure-and-mvp-slices.md` | `538610a144decd03b52f9e2cbc7584f5a07cdef726469643a8bdebcaaa069888` |
| 131 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase2/entity_authority_sources.csv` | `08-inventory-closure-and-mvp-slices.md` | `ce8be4c52dd720d0d5eb71ddfb8ba2bd96066ef7c51cf2ef1962949bc9046252` |
| 132 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase2/entity_fallback_contracts.csv` | `08-inventory-closure-and-mvp-slices.md` | `74973f4115fae56977de69751cdd839dd2bf380a24d2bae52a638226ace7e77c` |
| 133 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase2/owned_state.csv` | `08-inventory-closure-and-mvp-slices.md` | `082447ee1b88f6ba224f27e5191d5de5d3043bce5511203cb25337e7e5b1299c` |
| 134 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase2/owned_state_transitions.csv` | `08-inventory-closure-and-mvp-slices.md` | `53377426be02a48d9a312f12c5dcaa47564db296e296776daed89b3a25ec73bc` |
| 135 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase2/period_variants.csv` | `08-inventory-closure-and-mvp-slices.md` | `5cc9e8a2460066f2a5fbfdc5e6dd6965aef13c1357d42a61c5016f3adcae321f` |
| 136 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase2/profile_catalog_roles.csv` | `08-inventory-closure-and-mvp-slices.md` | `4888510776516af56ad467f49e943c213e4959f329209fea4b36cf62aaafa298` |
| 137 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase3/entities.csv` | `08-inventory-closure-and-mvp-slices.md` | `0703b0ad112829cf5e475d670aaf4f61bf59a0d301305e92657de3b32a5459b1` |
| 138 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase3/entity_authority_sources.csv` | `08-inventory-closure-and-mvp-slices.md` | `eb11917571b3872cac982e9be9333fe168b90516b79417da1ec30c3e8301402f` |
| 139 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase3/entity_fallback_contracts.csv` | `08-inventory-closure-and-mvp-slices.md` | `8252d4084bf2683584ab40c813c7082d2bbacfd5e5a32b7c386e5be9083398aa` |
| 140 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase3/owned_state.csv` | `08-inventory-closure-and-mvp-slices.md` | `d83985127a30aa452a059e01b6da0d14e2f16af402167e46fc0aeb83aebafa45` |
| 141 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase3/owned_state_transitions.csv` | `08-inventory-closure-and-mvp-slices.md` | `4d8ef7d818847a5098556ae22ad151cabe53296734f4b205d45772d5c9ecd20b` |
| 142 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase3/period_variants.csv` | `08-inventory-closure-and-mvp-slices.md` | `21cd858c4e2ed5dc613ce61819c0ff37f16df454b87be95ece7800ed15d70356` |
| 143 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase3/profile_catalog_roles.csv` | `08-inventory-closure-and-mvp-slices.md` | `3e017a12d6676069f6a8129e0deedd77581f97a232d2a867effad82d8e42382c` |
| 144 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase4/entity_authority_sources.csv` | `08-inventory-closure-and-mvp-slices.md` | `138782b4a9ff1b40dec04545c49f004777bb24d861d4f537c1a0c48c3aba7f9f` |
| 145 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase4/entity_fallback_contracts.csv` | `08-inventory-closure-and-mvp-slices.md` | `59c3b08697cbc338842e7f1698e8676669870a40ba0e2f4cec419a891b626ca0` |
| 146 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase4/owned_state.csv` | `08-inventory-closure-and-mvp-slices.md` | `507e08e72259697ec697a93b372e7cd271adda0b2d48a15e9428915257beb9c5` |
| 147 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase4/owned_state_transitions.csv` | `08-inventory-closure-and-mvp-slices.md` | `7ec54850f283671e2dbb1825cf254404feb735ddad4be7175888bf449c7a2fae` |
| 148 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase4/period_variants.csv` | `08-inventory-closure-and-mvp-slices.md` | `f88b193e7c7f2e976da0879a603ce74d7035745630dddaabf08239041f84e93e` |
| 149 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase5/entities.csv` | `08-inventory-closure-and-mvp-slices.md` | `6632bea5584aa2a9231be61c27379729f9dad4804c7d4b72d8279a53caf09982` |
| 150 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase5/entity_authority_sources.csv` | `08-inventory-closure-and-mvp-slices.md` | `92c733c311f781380e1b9839e6710199cf7a92777e9ec40578ec05ee5205bb40` |
| 151 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase5/entity_fallback_contracts.csv` | `08-inventory-closure-and-mvp-slices.md` | `99743dc88a94106ad6848dae409b551adf90fd3e9b814a87f41cc6a60c2ec34e` |
| 152 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase5/owned_state.csv` | `08-inventory-closure-and-mvp-slices.md` | `03ffb3a1a58cfd577d6cdee81c7f04dd280dbb1fdcb43dac39741a46ee834de4` |
| 153 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase5/owned_state_transitions.csv` | `08-inventory-closure-and-mvp-slices.md` | `c1c9eb18fb259135ab2a08acc5dc4236898ecc46e722064faf0c422c5b685322` |
| 154 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase5/period_variants.csv` | `08-inventory-closure-and-mvp-slices.md` | `589ccc79a74765f7c9dac9fcb4ad032fa6ba541cf90aff125c501702ae096d91` |
| 155 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase5/profile_catalog_roles.csv` | `08-inventory-closure-and-mvp-slices.md` | `5e815a30d87398008d79ae9eb0be9951298ab5a2b8c69f9fdf0a62720e1821de` |
| 156 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase6/owned_state.csv` | `08-inventory-closure-and-mvp-slices.md` | `df5611d7ecde9fafada0df6314bcee6b658b710670b30d19f37b8f4a400180ce` |
| 157 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/mvp_phase6/owned_state_transitions.csv` | `08-inventory-closure-and-mvp-slices.md` | `f4d5ce80e9d272436acdaebfb32b31fdfd2d7bd4a59bbdb85bf0afb9ab1c0d95` |
| 158 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/physical_products/entities.csv` | `09-world-inventories-media-and-interface.md` | `cc9feba0061d03ba854bdbd7008a95816f78618e9ecdcb2f33390210bc6d3f10` |
| 159 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/physical_products/product_channel_roles.csv` | `09-world-inventories-media-and-interface.md` | `60297aafd9061d2724899d909b51050de55818cdff9c63571336e60b826cb6c7` |
| 160 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/physical_products/product_families.csv` | `09-world-inventories-media-and-interface.md` | `bf1da89636276b393600f84854f6fc9fecd82bdce6b4d8d32bf8f44c2455c2ae` |
| 161 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/physical_products/transformations.csv` | `09-world-inventories-media-and-interface.md` | `655b4ffd148fd6eeda57e541a775586c66bb39163447f008f27e4c35635f4489` |
| 162 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/populations_organizations/entities.csv` | `09-world-inventories-media-and-interface.md` | `befe5283e1340b3aae65286aa8841324fd9259c12729042b0ce4f4f36f3883d5` |
| 163 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/sovereign_regional/entities.csv` | `09-world-inventories-media-and-interface.md` | `b2542700bbb5d96542b64bc79a3703f4d5723de37d8477fca98b34684560ad06` |
| 164 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/sovereign_regional/entity_fallback_contracts.csv` | `09-world-inventories-media-and-interface.md` | `2bb5d40ec0cee55251654e4d13deb033c5d41bb039670d4ff471ab81dabe973e` |
| 165 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/sovereign_regional/external_channel_providers.csv` | `09-world-inventories-media-and-interface.md` | `cc84ad7dfe18800c96910358b7581d6361f3b144ff0849448fc3a0d2392146d8` |
| 166 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/sovereign_regional/external_channels.csv` | `09-world-inventories-media-and-interface.md` | `90eb7d50b11b74c4a0dc0f735ab39ce3a724d08e669c20bc137b8b02b8d90b3e` |
| 167 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/sovereign_regional/external_market_channels.csv` | `09-world-inventories-media-and-interface.md` | `3fdd83d2e33338467f8bb31ba04d35fa91991508e60412ff31bbd7dfef49c175` |
| 168 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/sovereign_regional/market_interfaces.csv` | `09-world-inventories-media-and-interface.md` | `c4cbd3fad0babccdefdd042b3955fe1b02088eeab9912a1224549e7cd64ba2d1` |
| 169 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/sovereign_regional/observation_surfaces.csv` | `09-world-inventories-media-and-interface.md` | `8effe11479596c4319ded755d219bf863efc78df7908f79c008a4edc503096a2` |
| 170 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/sovereign_regional/provider_scopes.csv` | `09-world-inventories-media-and-interface.md` | `236d80edc79b0a6bc8324dce60ad19f840dcafd7cec7757b797deb53a62c40b1` |
| 171 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/sovereign_regional/residual_reconciliation.csv` | `09-world-inventories-media-and-interface.md` | `bc6e612325315d9e29202dbefd47781e138cc9149ef205b3fc6273ad48890afa` |
| 172 | `federal-reserve-chair-crisis-management-simulator/catalog/inventory/sovereign_regional/transmission_probes.csv` | `09-world-inventories-media-and-interface.md` | `d54130cd94a499e5eeb5a4cc3271e603a88a74c4ee50f3160330d427024b50b4` |
| 173 | `federal-reserve-chair-crisis-management-simulator/catalog/market_interfaces.csv` | `05-catalog-mechanisms-and-external-interfaces.md` | `d865db2483663696b902030f16083fb86c6d7e71af15c08052845d09af4105f0` |
| 174 | `federal-reserve-chair-crisis-management-simulator/catalog/observation_surfaces.csv` | `05-catalog-mechanisms-and-external-interfaces.md` | `8effe11479596c4319ded755d219bf863efc78df7908f79c008a4edc503096a2` |
| 175 | `federal-reserve-chair-crisis-management-simulator/catalog/owned_state.csv` | `04-catalog-core-ontology-and-validation.md` | `9433374a12bb9f0b002bca5f88359fe8091f59ef70c76abd8d1ddfcd9e731d8e` |
| 176 | `federal-reserve-chair-crisis-management-simulator/catalog/owned_state_transitions.csv` | `04-catalog-core-ontology-and-validation.md` | `2de6d5ee3fecca26ec31e7cae3607693f58e26d7af853348ef618929969374e1` |
| 177 | `federal-reserve-chair-crisis-management-simulator/catalog/period_variants.csv` | `04-catalog-core-ontology-and-validation.md` | `00fdaa5943b8cba252adaf865717ffcfea3eabcf0a054ccbd3e5d215c64880ba` |
| 178 | `federal-reserve-chair-crisis-management-simulator/catalog/presentation_refs.csv` | `04-catalog-core-ontology-and-validation.md` | `9ec8675f9948b81104bfbcc321c3bcdd5b207c4efee2cb6d56c8220068a9c10a` |
| 179 | `federal-reserve-chair-crisis-management-simulator/catalog/probe_coverage.csv` | `05-catalog-mechanisms-and-external-interfaces.md` | `ad9073a3b72f5ef0cf77d6cbc911fa0e83ca6bbd8460383657ca9b5f59c27cd4` |
| 180 | `federal-reserve-chair-crisis-management-simulator/catalog/product_channel_roles.csv` | `05-catalog-mechanisms-and-external-interfaces.md` | `f65ae4fc916157ea10b96950633690b7d64e3c430f6fea154682505cade09b99` |
| 181 | `federal-reserve-chair-crisis-management-simulator/catalog/product_families.csv` | `05-catalog-mechanisms-and-external-interfaces.md` | `bf1da89636276b393600f84854f6fc9fecd82bdce6b4d8d32bf8f44c2455c2ae` |
| 182 | `federal-reserve-chair-crisis-management-simulator/catalog/profile_catalog_roles.csv` | `04-catalog-core-ontology-and-validation.md` | `e7809ac53fb32f80bddbc70c2b091182f20cf1fbf22c93c4d31e8f5dc846911c` |
| 183 | `federal-reserve-chair-crisis-management-simulator/catalog/profile_required_offices.csv` | `04-catalog-core-ontology-and-validation.md` | `167347027482ab01b4e30f26bbaa1f27b74f0f597f66843a024fcd7027015426` |
| 184 | `federal-reserve-chair-crisis-management-simulator/catalog/provider_scopes.csv` | `05-catalog-mechanisms-and-external-interfaces.md` | `acdd57fe329d6ae44b549af976ed981364e6dc6b54a4f47ffcfa21821d3b3a73` |
| 185 | `federal-reserve-chair-crisis-management-simulator/catalog/relationships.csv` | `04-catalog-core-ontology-and-validation.md` | `15c70d3792e8d7bc558f72e411c78404d4e798bbf9f9fba1ed5aeab071bd30b6` |
| 186 | `federal-reserve-chair-crisis-management-simulator/catalog/research_backlog.csv` | `05-catalog-mechanisms-and-external-interfaces.md` | `cef339a3b25243d5eed41cc027f35b43de4de352be0c9ad9e1cfb8ff4502657a` |
| 187 | `federal-reserve-chair-crisis-management-simulator/catalog/residual_reconciliation.csv` | `05-catalog-mechanisms-and-external-interfaces.md` | `bc6e612325315d9e29202dbefd47781e138cc9149ef205b3fc6273ad48890afa` |
| 188 | `federal-reserve-chair-crisis-management-simulator/catalog/scenario_availability.csv` | `05-catalog-mechanisms-and-external-interfaces.md` | `c82bfdda600a910397fbfc7553a266b960acc0de9727f217aa9e6f9279a7078a` |
| 189 | `federal-reserve-chair-crisis-management-simulator/catalog/scenario_candidates.csv` | `05-catalog-mechanisms-and-external-interfaces.md` | `66c653ad1fa4db9500daf78f765d5c5d1ce09c1784f2851c288e4a32c393a3b4` |
| 190 | `federal-reserve-chair-crisis-management-simulator/catalog/schema.json` | `04-catalog-core-ontology-and-validation.md` | `5b3621d52b3c275c71a1f3531b29554caea785f7e3d7b67e76cd49ed90d0b320` |
| 191 | `federal-reserve-chair-crisis-management-simulator/catalog/test_catalog.py` | `04-catalog-core-ontology-and-validation.md` | `f672a7d48dbc88b743386d53a89942ca11107c966b7a95337edd9a095ef60d94` |
| 192 | `federal-reserve-chair-crisis-management-simulator/catalog/transformations.csv` | `05-catalog-mechanisms-and-external-interfaces.md` | `655b4ffd148fd6eeda57e541a775586c66bb39163447f008f27e4c35635f4489` |
| 193 | `federal-reserve-chair-crisis-management-simulator/catalog/transmission_probes.csv` | `05-catalog-mechanisms-and-external-interfaces.md` | `c0d096937c7bc0d07cf5726128102e7f417d145ad2c5942408946db9eb5a8c45` |
| 194 | `federal-reserve-chair-crisis-management-simulator/catalog/transmission_scenarios.csv` | `05-catalog-mechanisms-and-external-interfaces.md` | `bc895c16c7a777914529b24bcde07b910f00719b2396264f0755fb4e45b2fdcc` |
| 195 | `federal-reserve-chair-crisis-management-simulator/catalog/transmissions.csv` | `05-catalog-mechanisms-and-external-interfaces.md` | `7ef1af345e1cfdb40408cffb436309f6e59a612ab4a8b94b074a6af3672acd29` |
| 196 | `federal-reserve-chair-crisis-management-simulator/catalog/type_fidelity.csv` | `04-catalog-core-ontology-and-validation.md` | `1de105c844730515754d09e2636771f6bac73a839c6dca843597fbae35f10426` |
| 197 | `federal-reserve-chair-crisis-management-simulator/catalog/types.csv` | `04-catalog-core-ontology-and-validation.md` | `81c8936447febfeddd54a51d7037ff9fe824881bffc6e280a511cf92e8b3d5a5` |
| 198 | `federal-reserve-chair-crisis-management-simulator/catalog/world_profiles.csv` | `04-catalog-core-ontology-and-validation.md` | `51863bb6ec6061278abbcd44adf932e6de2c9084111ba980f3b0876a9c82c80f` |
| 199 | `federal-reserve-chair-crisis-management-simulator/pr-description.md` | `01-foundations-and-research.md` | `9c1b189439ca976e8a6f10711cece190caef5cbb22eb3a1de95530836e9463ed` |
| 200 | `federal-reserve-chair-crisis-management-simulator/task.md` | `01-foundations-and-research.md` | `bbbdd49403ad5d71e91fe689001ee1b7653b248fece2406335bdf359a8c61e82` |
| 201 | `federal-reserve-chair-crisis-simulator-game/task.md` | `01-foundations-and-research.md` | `c2993fd675935bae13a853b5df3a2a7f3b18565d07ffeabd79a724757ec95459` |

## Exceptional cases

- No artifact required a standalone bundle: the largest source payload is 287,333 bytes.
- `06-generated-planning-and-research-status.md` and `04-catalog-core-ontology-and-validation.md` exceed the preferred 600 KB target because their generated or implementation/ontology records are strongly coupled; both remain below the roughly 750 KB ceiling.

## Verification

- Source artifact count: 201.
- Output artifact count: 201.
- Each provenance-qualified artifact identity maps to exactly one bundle.
- Every source SHA-256 metadata value is preserved in its corresponding artifact section and mapping entry (201 of 201; 183 distinct values).
- Bundle construction preserves original artifact headings, YAML metadata, and payload bytes; only outer Markdown fence delimiters are reselected collision-safely to avoid inherited delimiter expansion.
