<?php

namespace SignatureVersions80;

interface A {}
final class F {}

function defaults($a = 1, $b, ?int $c = null, $d, int|null $e = null, $f, int $g = null): void {}

function intersection(A&F $a): A&F {}

class Holder
{
	public A&F $property;
}
