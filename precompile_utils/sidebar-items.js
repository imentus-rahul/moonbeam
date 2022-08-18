initSidebarItems({"attr":[["generate_function_selector","This macro allows to associate to each variant of an enumeration a discriminant (of type u32 whose value corresponds to the first 4 bytes of the Hash Keccak256 of the character string indicated by the user of this macro."]],"fn":[["encoded_revert",""],["revert","Generated a `PrecompileFailure::Revert` with proper encoding for the output. If the revert needs improved formatting such as backtraces, `Revert` type should be used instead."],["succeed",""]],"macro":[["assert_event_emitted","Panics if an event is not found in the system log of events"],["assert_event_not_emitted",""],["keccak256",""],["read_args","Helper to read arguments of a Solidity function. Arguments are read in the provided order using the provided types. Those types should match the ones in the Solidity file, and identifiers used should match Solidity ones."],["read_struct","Helper to write `EvmData` impl for Solidity structs. Identifiers used should match Solidity ones. Types are infered from context, which should always be possible when parsing input to build a Rust struct."]],"mod":[["costs","Cost calculations. TODO: PR EVM to make those cost calculations public."],["data",""],["handle",""],["logs",""],["modifier","Provide checks related to function modifiers (view/payable)."],["precompile_set","Provide utils assemble precompiles and precompilesets into a final precompile set with security checks. All security checks are enabled by default and must be disabled explicely throught type annotations."],["prelude",""],["revert","Utilities to work with revert messages with support for backtraces and consistent formatting."],["solidity","Utility module to interact with solidity file."],["substrate","Utils related to Substrate features:"],["testing",""]],"trait":[["Precompile","One single precompile used by EVM engine."],["StatefulPrecompile","Trait similar to `fp_evm::Precompile` but with a `&self` parameter to manage some state (this state is only kept in a single transaction and is lost afterward)."]],"type":[["EvmResult","Alias for Result returning an EVM precompile error."]]});