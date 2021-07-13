#@local 
gap> START_TEST("error.tst");
gap> LoadPackage("vole", false);
true

#
gap> Vole.FindOne([VoleRefiner.SetStab([3,4,5,[2,3]])], rec(points := 7));
Error, There was a fatal error in vole: Invalid problem specification. Does on\
e of your constraints have the wrong argument type?
gap> Vole.FindGroup([VoleCon.Stabilize([2,3,4], OnSetsSets)], rec(points := 7));
Error, There was a fatal error in vole: Invalid problem specification. Does on\
e of your constraints have the wrong argument type?

#
gap> STOP_TEST("error.tst");
