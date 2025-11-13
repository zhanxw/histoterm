Install 

  cargo install --path .


Build

  cargo build


Example

  # Example: numbers 1..100 with some repeats
  seq 100 | awk 'BEGIN{srand()} {for(i=0;i<int(rand()*3);i++) print $1}' \
    | ./target/release/histoterm --bins 12 --width 60
