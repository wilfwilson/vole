#@local
gap> START_TEST("settupletrans-coset.tst");
gap> ReadPackage("vole", "tst/test_functions.g");
true

#
gap> QC_Check([ QC_SetOf(QC_ListOf(IsPosInt)), IsPermGroup ], function(s,g)
>      local s2, max, res, p, grp;
>      max := Maximum(Maximum(Flat(s)), LargestMovedPoint(g),2);
>      p := Random(g);
>      s2 := OnSetsTuples(s,p);
>      grp := Stabilizer(g, s, OnSetsTuples);
>      res := VoleFind.Coset([VoleCon.Transport(s,s2,OnSetsTuples), BTKit_Con.InGroupSimple(max, g)]);
>      if res <> RightCoset(grp, p) then
>          return StringFormatted("Failure: {} {} {}", s2, p, res);
>      fi;
>      return true;
>  end);
true

#
gap> STOP_TEST("settupletrans.tst");