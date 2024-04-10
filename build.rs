fn main() {
    cxx_build::bridge("src/main.rs")
        .file("myFibSeqProgram/fibSeq.cpp")
        .flag_if_supported("-std=c++14")
        .compile("fibonacci");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=myFibSeqProgram/fibSeq.cpp");
    println!("cargo:rerun-if-changed=myFibSeqProgram/fibSeq.h");
}
