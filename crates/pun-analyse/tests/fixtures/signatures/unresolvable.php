<?php

namespace SignatureUnresolvable;

interface I {}
interface J {}
class C {}
class D extends C {}
final class F {}
final class G implements I {}
trait T {}
enum E {}

class Holder
{
	public I&F $finalUnrelated;
	public I&C $interfaceAndClass;
	public C&D $subclass, $second;
	public I&G $finalImplementing;
	public I&T $trait;
	public I&E $enum;
	public \Countable&\ArrayAccess $builtins;
	public \stdClass&\Exception $builtinClasses;
	public \Countable&C $builtinInterface;
	public I&Unknown $unknown;
	public (C&F)|(I&J) $partly;
	public (C&F)|(I&F) $allNever;
	public self&I $self;
	public object&I $object;
	public iterable&I $iterable;
	public I&J&int $threeMembers;

	public function __construct(
		public C&F $promoted,
		public C&F $promotedNull = null,
	) {}

	public function params(C&F $a, C&F $b = null, C&F ...$c): self&F {}
}

$f = function (C&F $a = null): C&F {};
