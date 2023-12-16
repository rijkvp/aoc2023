alias c := comp
alias r := run
alias t := test
alias b := bench

tmpdir := `mktemp -d`

# compile a Rust file
[no-cd]
comp file:
    rustc {{file}} -C opt-level=3 -o {{tmpdir}}/{{file_name(file)}}

# run a rust file
[no-cd]
run file: (comp file)
    {{tmpdir}}/{{file_name(file)}}

# comple & run with input
[no-cd]
test file: (comp file)
    {{tmpdir}}/{{file_name(file)}} < input

# comple & run with input & time
[no-cd]
bench file: (comp file)
    time {{tmpdir}}/{{file_name(file)}} < input
