#@local p
gap> START_TEST("nothing-transport.tst");
gap> LoadPackage("vole", false);;
gap> Vole.FindOne([BTKit_Con.Nothing()] : conf := rec(points := 6));
fail
gap> Vole.FindOne([BTKit_Con.Nothing2()] : conf := rec(points := 6));
fail

#
gap> STOP_TEST("nothing-transport.tst");
