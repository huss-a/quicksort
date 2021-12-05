for _ in {0..500}; do
	cargo run --release --quiet
	if [[ $? == 0 ]]; then
		echo OK
	else
		echo FAIL
	fi
done
