<?php // lint >= 8.5

namespace ConstantAttributes;

use Attribute;

#[Attribute(Attribute::TARGET_CLASS | Attribute::TARGET_CONSTANT)]
class MyAttr
{

}

#[Attribute(Attribute::TARGET_ALL)]
class MyAttrAll
{

}

#[Attribute(Attribute::TARGET_CLASS)]
class IncompatibleAttr
{

}

#[MyAttr]
const One = 1;

#[MyAttrAll]
const Two = 1;

#[IncompatibleAttr]
const Three = 1;

#[MyAttr(1)]
const Four = 1;

const WithoutAttributes = 1;
