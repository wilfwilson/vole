gap> Read("gap-code/vole-base.g");
gap> Read("gap-code/tst/test_functions.g");
gap> LoadPackage("quickcheck", false);;
gap> QC_Check([IsPermGroup, QC_SetOf(QC_SetOf(IsPosInt))],
>    {g, s} -> VoleTestCanonical(Maximum([LargestMovedPoint(g), Maximum(Flat(s)),2]), g, s, con.SetSetStab, OnSetsSets));
true