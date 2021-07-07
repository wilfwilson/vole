






Vole.Intersection := function(grps...)
    if ForAny(grps, not IsPermGroup) then
        ErrorNoReturn("Vole.Intersection: Input must be a list of perm groups");
    fi;
    if IsEmpty(grps) then
        ErrorNoReturn("Vole.Intersection: Input must be a non-empty list");
    fi;
    return Vole.FindGroup(List(grps, VoleCon.InGroup));
end;

Vole.Normaliser := function(G, H)
    if not IsPermGroup(G) then
        ErrorNoReturn("Vole.Normaliser: First argument must be a perm group");
    fi;

    if IsPermGroup(H) then
        return Vole.FindGroup([VoleCon.InGroup(G), VoleCon.Normalise(H)]);
    elif IsPerm(H) then
        return Vole.FindGroup([VoleCon.InGroup(G), VoleCon.Centraliser(H)]);
    else
        ErrorNoReturn("Vole.Normaliser: Second argument must a perm group or permutation");
    fi;
end;

Vole.IsConjugate := function(G, x, y)
    if not IsPermGroup(G) then
        ErrorNoReturn("Vole.Normaliser: First argument must be a perm group");
    fi;
    if not (IsPerm(x) and IsPerm(y)) then
        ErrorNoReturn("Vole.Normaliser: Second and Third arguments must be permutations");
    fi;
    Vole.FindOne([VoleCon.InGroup(G), VoleCon.Conjugate(x,y)]);
end;