_path=./target/PassworldCrack
_lib=./target/lib/libCrack.so
_file=./src/main.c
main:
	gcc --debug $(_file) -o $(_path)

lib:
	gcc --debug $(_file) -fPIC -shared -o $(_lib)
cleanALL:
	rm -rf $(_path)
	rm -rf $(_lib)
