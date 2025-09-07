

//  Input: a 2-torsion point b in the Jacobian of a hyperelliptic curve in characteristic p
// Output: the p^2-Frobenius conjugate of b in the Jacobian
function p2_conjugate(b)
    f := b[1];
    p := Characteristic(Parent(f)); q := p^2;
    c := Polynomial([coef^q: coef in Coefficients(b[1])]);
    return Parent(b)![c,0];
end function;

procedure count_Fp2_types(p, inv_list)
	num_ord4 := 0; num_type1 := 0; num_type2 := 0;
	for inv in inv_list do
		C2 := HyperellipticCurveFromG2Invariants(ChangeUniverse(inv,GF(p^2)));
		J2 := Jacobian(C2);
		G2,map2 := TwoTorsionSubgroup(J2);
		num_ord4 +:= 1;
		J4 := BaseChange(J2,GF(p^4)); G4,map4 := TwoTorsionSubgroup(J4);
		b1 := J4!map2(G2.1); b2 := J4!map2(G2.2);
		assert p2_conjugate(b1) eq b1;      // check if b1 is Fp2-rational.
		assert p2_conjugate(b2) eq b2;      // check if b2 is Fp2-rational.
		assert WeilPairing(b1,b2,2) eq 1;   // check if e2(b1,b2) = 1.
		repeat
			g3 := Random(G4);
			if WeilPairing(b1,map4(g3),2) eq -1 and WeilPairing(b2,map4(g3),2) eq 1 then
				b3 := map4(g3); break;
			end if;
		until false;
		repeat
			g4 := Random(G4);
			if WeilPairing(b1,map4(g4),2) eq 1 and WeilPairing(b2,map4(g4),2) eq -1 and WeilPairing(b3,map4(g4),2) eq 1 then
				b4 := map4(g4); break;
			end if;
		until false;
		b := [b1,b2,b3,b4];
		assert p2_conjugate(b3)-b3 in [b1,b2,b1+b2]; // check if b3 is NOT Fp2-rational but pi(b3)-b3 is Fp2-rational.
		assert p2_conjugate(b4)-b4 in [b1,b2,b1+b2]; // check if b4 is NOT Fp2-rational but pi(b4)-b4 is Fp2-rational.
		if p2_conjugate(b3)-b3 eq b1 and p2_conjugate(b4)-b4 eq b2 then
			num_type1 +:= 1;
		elif p2_conjugate(b3)-b3 eq b2 and p2_conjugate(b4)-b4 eq b1 then
			num_type2 +:= 1;
		elif p2_conjugate(b3)-b3 eq b1+b2 and p2_conjugate(b4)-b4 eq b1 then
			num_type1 +:= 1;
		else
			num_type1 +:= 1;
		end if;
	end for;

	if p mod 4 eq 1 then
		conjN1 := (p + 1)/2 * ((p - 1)/4)^2;
		conjN2 := 2/3 * (p + 1)/2 * ((p - 1)/4)^2;
	else
		conjN1 := (p - 1)/2 * ((p + 1)/4)^2;
		conjN2 := 0;
	end if;
	printf "N1 = %o, N2 = %o, conjecgture is %o\n",num_type1, num_type2, num_type1 eq conjN1 and num_type2 eq conjN2;
end procedure;
