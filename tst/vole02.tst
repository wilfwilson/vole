# Vole, chapter 2
#
# DO NOT EDIT THIS FILE - EDIT EXAMPLES IN THE SOURCE INSTEAD!
#
# This file has been generated by AutoDoc. It contains examples extracted from
# the package documentation. Each example is preceded by a comment which gives
# the name of a GAPDoc XML file and a line range from which the example were
# taken. Note that the XML file in turn may have been generated by AutoDoc
# from some other input.
#
gap> START_TEST("vole02.tst");

# doc/_Chapter_install.xml:224-227
gap> LoadPackage("vole", false);
true

# doc/_Chapter_install.xml:246-253
gap> A4 := AlternatingGroup(4);
Alt( [ 1 .. 4 ] )
gap> D8 := DihedralGroup(IsPermGroup, 8);
Group([ (1,2,3,4), (2,4) ])
gap> Vole.Intersection(A4, D8) = Group([ (1,2)(3,4), (1,4)(2,3) ]);
true

#
gap> STOP_TEST("vole02.tst", 1);
