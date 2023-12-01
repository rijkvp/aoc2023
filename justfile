alias c := comp
alias r := run
alias t := test

tmpdir := `mktemp -d`

# compile a Rust file
[no-cd]
comp file:
    rustc {{file}} -o {{tmpdir}}/{{file_name(file)}}

# run a rust file
[no-cd]
run file: (comp file)
    {{tmpdir}}/{{file_name(file)}}

# test a rust file with input
[no-cd]
test file: (comp file)
    {{tmpdir}}/{{file_name(file)}} < input
