var searchIndex = JSON.parse('{\
"badread":{"doc":"","i":[[5,"main","badread","",null,[[],["result",6]]]],"p":[]},\
"badread_rs":{"doc":"","i":[[0,"alignment","badread_rs","Function to align sequence",null,null],[5,"edit_distance","badread_rs::alignment","",null,[[],["u64",15]]],[5,"identity","","",null,[[],["f64",15]]],[0,"cli","badread_rs","All stuff relate to command line",null,null],[0,"simulate","badread_rs::cli","All stuff relate to simulate subcommand",null,null],[3,"Quantity","badread_rs::cli::simulate","Store quantity as coverage of number of base",null,null],[11,"number_of_base","","Convert Quantity in a number of base, if base is set …",0,[[["u64",15]],["u64",15]]],[3,"Duo","","Store a pair of value, can be parse from str if it\'s …",null,null],[3,"Trio","","Store a trio of value, can be parse from str if it\'s …",null,null],[3,"Command","","Struct use to parse simulate subcommand argument",null,null],[12,"reference_path","","Path to reference sequence in fasta format",1,null],[12,"quantity","","Quantity of base badread have to generate",1,null],[12,"length","","Read length distribution parameter",1,null],[12,"identity","","Identity distribution parameter",1,null],[12,"error_model","","Error model used",1,null],[12,"qscore_model","","Qualtity score model used",1,null],[12,"seed","","Seed used",1,null],[12,"start_adapter","","Start adapter parameter",1,null],[12,"end_adapter","","End adapter parameter",1,null],[12,"start_adapter_seq","","Start adapter sequence",1,null],[12,"end_adapter_seq","","End adapter sequence",1,null],[12,"junk_reads","","Junk reads parameter",1,null],[12,"random_reads","","Random reads parameter",1,null],[12,"chimeras","","Chimeras parameter",1,null],[12,"glitches","","Glitches parameter",1,null],[12,"small_plasmid_bias","","Small plasmid bias or not",1,null],[3,"Command","badread_rs::cli","Struct use to parse argument from command line",null,null],[12,"subcmd","","Subcommand call",2,null],[12,"threads","","Number of thread badread_rs can use",2,null],[12,"verbosity","","Verbosity level",2,null],[4,"SubCommand","","Enum to manage subcommand polymorphism",null,null],[13,"Simulate","","",3,null],[5,"i82level","","Convert verbosity level (number of v) is log::Level",null,[[["i8",15]],[["level",4],["option",4]]]],[5,"set_nb_threads","","set number of global rayon thread pool",null,[[["usize",15]]]],[0,"error","badread_rs","All stuff relate to error",null,null],[0,"cli","badread_rs::error","Command line interface error",null,null],[4,"Cli","badread_rs::error::cli","Enum to manage error polymorphism",null,null],[13,"CantParssQuantity","","quantity didn\'t match to pattern \\\\d+[KMGx]?",4,null],[13,"CantParseDuo","","Cant parse a duo of value",4,null],[13,"CantParseTrio","","Cant parse a trio of value",4,null],[0,"model","badread_rs::error","Model error",null,null],[4,"Model","badread_rs::error::model","Enum to manage error polymorphism",null,null],[13,"ErrorModelParsing","","Error durring error model parsing",5,null],[13,"QualityModelParsing","","Error durring quality model parsing",5,null],[13,"QualityModelNotMinimalCigarString","","Quality model not contains minimal cigar string",5,null],[13,"QualityModelCigarLenNotOdd","","Cigar string length must be odd",5,null],[4,"Error","badread_rs::error","Enum to manage error polymorphism",null,null],[13,"Cli","","Error related to command line interface",6,null],[13,"Model","","Error related to model",6,null],[0,"model","badread_rs","Manage model",null,null],[0,"error","badread_rs::model","Model to add error sequence",null,null],[3,"Error","badread_rs::model::error","Struct to load and apply error model",null,null],[11,"from_stream","","Load model from an stdin",7,[[],["result",6]]],[11,"add_errors_to_kmer","","Add error to a kmer",7,[[]]],[5,"random_error","","Add a single random error in a kmer",null,[[],[["vec",3],["u8",15]]]],[0,"quality","badread_rs::model","Model to generate qscore",null,null],[3,"Quality","badread_rs::model::quality","Struct to load and apply quality model",null,null],[11,"from_stream","","Load model from an stdin",8,[[],["result",6]]],[11,"get_qscore","","Generate error associate to a cigar string with odd length",8,[[],[["result",6],["u8",15]]]],[0,"references","badread_rs","A collections of sequence to store reference sequence",null,null],[6,"References","badread_rs::references","A collections of sequence",null,null],[8,"AbsReferences","","",null,null],[10,"from_stream","","",9,[[]]],[5,"random_base","badread_rs","Get a random base",null,[[],["u8",15]]],[5,"random_base_diff","","Get a random base diffrent than nuc",null,[[["u8",15]],["u8",15]]],[11,"from","badread_rs::cli::simulate","",0,[[]]],[11,"into","","",0,[[]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"vzip","","",0,[[]]],[11,"to_subset","","",0,[[],["option",4]]],[11,"is_in_subset","","",0,[[],["bool",15]]],[11,"to_subset_unchecked","","",0,[[]]],[11,"from_subset","","",0,[[]]],[11,"init","","",0,[[],["usize",15]]],[11,"deref","","",0,[[["usize",15]]]],[11,"deref_mut","","",0,[[["usize",15]]]],[11,"drop","","",0,[[["usize",15]]]],[11,"from","","",10,[[]]],[11,"into","","",10,[[]]],[11,"borrow","","",10,[[]]],[11,"borrow_mut","","",10,[[]]],[11,"try_from","","",10,[[],["result",4]]],[11,"try_into","","",10,[[],["result",4]]],[11,"type_id","","",10,[[],["typeid",3]]],[11,"vzip","","",10,[[]]],[11,"to_subset","","",10,[[],["option",4]]],[11,"is_in_subset","","",10,[[],["bool",15]]],[11,"to_subset_unchecked","","",10,[[]]],[11,"from_subset","","",10,[[]]],[11,"init","","",10,[[],["usize",15]]],[11,"deref","","",10,[[["usize",15]]]],[11,"deref_mut","","",10,[[["usize",15]]]],[11,"drop","","",10,[[["usize",15]]]],[11,"from","","",11,[[]]],[11,"into","","",11,[[]]],[11,"borrow","","",11,[[]]],[11,"borrow_mut","","",11,[[]]],[11,"try_from","","",11,[[],["result",4]]],[11,"try_into","","",11,[[],["result",4]]],[11,"type_id","","",11,[[],["typeid",3]]],[11,"vzip","","",11,[[]]],[11,"to_subset","","",11,[[],["option",4]]],[11,"is_in_subset","","",11,[[],["bool",15]]],[11,"to_subset_unchecked","","",11,[[]]],[11,"from_subset","","",11,[[]]],[11,"init","","",11,[[],["usize",15]]],[11,"deref","","",11,[[["usize",15]]]],[11,"deref_mut","","",11,[[["usize",15]]]],[11,"drop","","",11,[[["usize",15]]]],[11,"from","","",1,[[]]],[11,"into","","",1,[[]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"vzip","","",1,[[]]],[11,"to_subset","","",1,[[],["option",4]]],[11,"is_in_subset","","",1,[[],["bool",15]]],[11,"to_subset_unchecked","","",1,[[]]],[11,"from_subset","","",1,[[]]],[11,"init","","",1,[[],["usize",15]]],[11,"deref","","",1,[[["usize",15]]]],[11,"deref_mut","","",1,[[["usize",15]]]],[11,"drop","","",1,[[["usize",15]]]],[11,"from","badread_rs::cli","",2,[[]]],[11,"into","","",2,[[]]],[11,"borrow","","",2,[[]]],[11,"borrow_mut","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"try_into","","",2,[[],["result",4]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"vzip","","",2,[[]]],[11,"to_subset","","",2,[[],["option",4]]],[11,"is_in_subset","","",2,[[],["bool",15]]],[11,"to_subset_unchecked","","",2,[[]]],[11,"from_subset","","",2,[[]]],[11,"init","","",2,[[],["usize",15]]],[11,"deref","","",2,[[["usize",15]]]],[11,"deref_mut","","",2,[[["usize",15]]]],[11,"drop","","",2,[[["usize",15]]]],[11,"from","","",3,[[]]],[11,"into","","",3,[[]]],[11,"borrow","","",3,[[]]],[11,"borrow_mut","","",3,[[]]],[11,"try_from","","",3,[[],["result",4]]],[11,"try_into","","",3,[[],["result",4]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"vzip","","",3,[[]]],[11,"to_subset","","",3,[[],["option",4]]],[11,"is_in_subset","","",3,[[],["bool",15]]],[11,"to_subset_unchecked","","",3,[[]]],[11,"from_subset","","",3,[[]]],[11,"init","","",3,[[],["usize",15]]],[11,"deref","","",3,[[["usize",15]]]],[11,"deref_mut","","",3,[[["usize",15]]]],[11,"drop","","",3,[[["usize",15]]]],[11,"from","badread_rs::error::cli","",4,[[]]],[11,"into","","",4,[[]]],[11,"to_string","","",4,[[],["string",3]]],[11,"borrow","","",4,[[]]],[11,"borrow_mut","","",4,[[]]],[11,"try_from","","",4,[[],["result",4]]],[11,"try_into","","",4,[[],["result",4]]],[11,"type_id","","",4,[[],["typeid",3]]],[11,"vzip","","",4,[[]]],[11,"to_subset","","",4,[[],["option",4]]],[11,"is_in_subset","","",4,[[],["bool",15]]],[11,"to_subset_unchecked","","",4,[[]]],[11,"from_subset","","",4,[[]]],[11,"init","","",4,[[],["usize",15]]],[11,"deref","","",4,[[["usize",15]]]],[11,"deref_mut","","",4,[[["usize",15]]]],[11,"drop","","",4,[[["usize",15]]]],[11,"from","badread_rs::error::model","",5,[[]]],[11,"into","","",5,[[]]],[11,"to_string","","",5,[[],["string",3]]],[11,"borrow","","",5,[[]]],[11,"borrow_mut","","",5,[[]]],[11,"try_from","","",5,[[],["result",4]]],[11,"try_into","","",5,[[],["result",4]]],[11,"type_id","","",5,[[],["typeid",3]]],[11,"vzip","","",5,[[]]],[11,"to_subset","","",5,[[],["option",4]]],[11,"is_in_subset","","",5,[[],["bool",15]]],[11,"to_subset_unchecked","","",5,[[]]],[11,"from_subset","","",5,[[]]],[11,"init","","",5,[[],["usize",15]]],[11,"deref","","",5,[[["usize",15]]]],[11,"deref_mut","","",5,[[["usize",15]]]],[11,"drop","","",5,[[["usize",15]]]],[11,"from","badread_rs::error","",6,[[]]],[11,"into","","",6,[[]]],[11,"to_string","","",6,[[],["string",3]]],[11,"borrow","","",6,[[]]],[11,"borrow_mut","","",6,[[]]],[11,"try_from","","",6,[[],["result",4]]],[11,"try_into","","",6,[[],["result",4]]],[11,"type_id","","",6,[[],["typeid",3]]],[11,"vzip","","",6,[[]]],[11,"to_subset","","",6,[[],["option",4]]],[11,"is_in_subset","","",6,[[],["bool",15]]],[11,"to_subset_unchecked","","",6,[[]]],[11,"from_subset","","",6,[[]]],[11,"init","","",6,[[],["usize",15]]],[11,"deref","","",6,[[["usize",15]]]],[11,"deref_mut","","",6,[[["usize",15]]]],[11,"drop","","",6,[[["usize",15]]]],[11,"from","badread_rs::model::error","",7,[[]]],[11,"into","","",7,[[]]],[11,"borrow","","",7,[[]]],[11,"borrow_mut","","",7,[[]]],[11,"try_from","","",7,[[],["result",4]]],[11,"try_into","","",7,[[],["result",4]]],[11,"type_id","","",7,[[],["typeid",3]]],[11,"vzip","","",7,[[]]],[11,"to_subset","","",7,[[],["option",4]]],[11,"is_in_subset","","",7,[[],["bool",15]]],[11,"to_subset_unchecked","","",7,[[]]],[11,"from_subset","","",7,[[]]],[11,"init","","",7,[[],["usize",15]]],[11,"deref","","",7,[[["usize",15]]]],[11,"deref_mut","","",7,[[["usize",15]]]],[11,"drop","","",7,[[["usize",15]]]],[11,"from","badread_rs::model::quality","",8,[[]]],[11,"into","","",8,[[]]],[11,"borrow","","",8,[[]]],[11,"borrow_mut","","",8,[[]]],[11,"try_from","","",8,[[],["result",4]]],[11,"try_into","","",8,[[],["result",4]]],[11,"type_id","","",8,[[],["typeid",3]]],[11,"vzip","","",8,[[]]],[11,"to_subset","","",8,[[],["option",4]]],[11,"is_in_subset","","",8,[[],["bool",15]]],[11,"to_subset_unchecked","","",8,[[]]],[11,"from_subset","","",8,[[]]],[11,"init","","",8,[[],["usize",15]]],[11,"deref","","",8,[[["usize",15]]]],[11,"deref_mut","","",8,[[["usize",15]]]],[11,"drop","","",8,[[["usize",15]]]],[11,"from_stream","badread_rs","",12,[[]]],[11,"from","badread_rs::error","",6,[[["cli",4]]]],[11,"from","","",6,[[["model",4]]]],[11,"eq","badread_rs::cli::simulate","",0,[[["quantity",3]],["bool",15]]],[11,"ne","","",0,[[["quantity",3]],["bool",15]]],[11,"eq","","",10,[[["duo",3]],["bool",15]]],[11,"ne","","",10,[[["duo",3]],["bool",15]]],[11,"eq","","",11,[[["trio",3]],["bool",15]]],[11,"ne","","",11,[[["trio",3]],["bool",15]]],[11,"fmt","","",0,[[["formatter",3]],["result",6]]],[11,"fmt","","",10,[[["formatter",3]],["result",6]]],[11,"fmt","","",11,[[["formatter",3]],["result",6]]],[11,"fmt","","",1,[[["formatter",3]],["result",6]]],[11,"fmt","badread_rs::cli","",2,[[["formatter",3]],["result",6]]],[11,"fmt","","",3,[[["formatter",3]],["result",6]]],[11,"fmt","badread_rs::error::cli","",4,[[["formatter",3]],["result",6]]],[11,"fmt","badread_rs::error::model","",5,[[["formatter",3]],["result",6]]],[11,"fmt","badread_rs::error","",6,[[["formatter",3]],["result",6]]],[11,"fmt","badread_rs::error::cli","",4,[[["formatter",3]],["result",6]]],[11,"fmt","badread_rs::error::model","",5,[[["formatter",3]],["result",6]]],[11,"fmt","badread_rs::error","",6,[[["formatter",3]],["result",6]]],[11,"from_str","badread_rs::cli::simulate","",0,[[["str",15]],["result",4]]],[11,"from_str","","",10,[[["str",15]],["result",4]]],[11,"from_str","","",11,[[["str",15]],["result",4]]],[11,"source","badread_rs::error","",6,[[],[["option",4],["error",8]]]],[11,"into_app","badread_rs::cli::simulate","",1,[[],["app",3]]],[11,"augment_clap","","",1,[[["app",3]],["app",3]]],[11,"into_app","badread_rs::cli","",2,[[],["app",3]]],[11,"augment_clap","","",2,[[["app",3]],["app",3]]],[11,"into_app","","",3,[[],["app",3]]],[11,"augment_clap","","",3,[[["app",3]],["app",3]]],[11,"from_arg_matches","badread_rs::cli::simulate","",1,[[["argmatches",3]]]],[11,"from_arg_matches","badread_rs::cli","",2,[[["argmatches",3]]]],[11,"from_arg_matches","","",3,[[["argmatches",3]]]],[11,"augment_subcommands","","",3,[[["app",3]],["app",3]]],[11,"from_subcommand","","",3,[[["option",4]],["option",4]]]],"p":[[3,"Quantity"],[3,"Command"],[3,"Command"],[4,"SubCommand"],[4,"Cli"],[4,"Model"],[4,"Error"],[3,"Error"],[3,"Quality"],[8,"AbsReferences"],[3,"Duo"],[3,"Trio"],[6,"References"]]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);