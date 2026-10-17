<?php // lint >= 8.1

namespace DeprecatedAttrOnClass;

use Deprecated;

#[Deprecated]
class Foo
{

}

#[Deprecated]
interface Bar
{

}

#[Deprecated]
enum Baz
{

}
