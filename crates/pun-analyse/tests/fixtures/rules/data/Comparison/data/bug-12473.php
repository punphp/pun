<?php

namespace Bug12473;

use ReflectionClass;

class Picture
{
}

class PictureUser extends Picture
{
}

class PictureProduct extends Picture
{
}

function getPictureFqn(string $pictureType): ?string
{
	/** @var class-string<Picture|object> $fqn */
	$fqn = $pictureType;
	if ($fqn === Picture::class) {
		return Picture::class;
	}
	$refl = new \ReflectionClass($fqn);
	if (!$refl->isSubclassOf(Picture::class)) {
		return null;
	}

	return $fqn;
}

/**
 * @param class-string<Picture> $a
 */
function doFoo(string $a): void {
	$r = new ReflectionClass($a);
	if ($r->isSubclassOf(Picture::class)) {

	}
}

/**
 * @param class-string<PictureUser> $a
 */
function doFoo2(string $a): void {
	$r = new ReflectionClass($a);
	if ($r->isSubclassOf(PictureProduct::class)) {

	}
}

/**
 * @param class-string<PictureUser> $a
 */
function doFoo3(string $a): void {
	$r = new ReflectionClass($a);
	if ($r->isSubclassOf(PictureUser::class)) {

	}
}

/**
 * @param ReflectionClass<object> $a
 * @param class-string<object> $b
 * @return void
 */
function doFoo4(ReflectionClass $a, string $b): void {
	if ($a->isSubclassOf($b)) {

	}
};
