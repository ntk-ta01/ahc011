for val in (seq 0 9)
  echo $val
  echo $val >> result.txt
  cargo run --release --bin ahc011-a < ./tools/in/000$val.txt ^^result.txt
end

for val in (seq 10 99)
  echo $val
  echo $val >> result.txt
  cargo run --release --bin ahc011-a < ./tools/in/00$val.txt ^^result.txt
end
echo $score