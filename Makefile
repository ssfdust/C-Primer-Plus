all:
	find . -not -path "./.git/*" -type f -name "*.rs" -execdir rustc -C debuginfo=0 -C opt-level=3 \{\} \;

clean:
	find . -not -path "./.git/*" -type f -perm -u=x -exec rm -f \{\} \;
