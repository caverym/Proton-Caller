build:
	g++ -o proton-call src/ProtonCaller.cpp src/ProtonArguments.cpp -std=c++17

install:
	cp ./proton-call usr/bin/
	mkdir -p /usr/share/licenses/proton-call/
	cp ./LICENSE /usr/share/licenses/proton-call/
	install -g 0 -o 0 -m 0644 manual/proton-call.6 /usr/share/man/man6/
	gzip -f usr/share/man/man6/proton-call.6

uninstall:
	rm -f ./proton-call usr/bin/proton-call
	rm -rf /usr/share/licenses/proton-call/
	rm -f /usr/share/man/man6/proton-call.6.gz
