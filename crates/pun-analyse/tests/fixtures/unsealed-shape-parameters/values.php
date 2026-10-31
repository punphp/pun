<?php
declare(strict_types=1);

namespace UnsealedShapeParameterValues;

/** @param array{int, ...<int>} $shape */
function numericPrefix(array $shape): void { \PHPStan\dumpType($shape['tail']); }

/** @param array{head: bool, ...<string>} $shape */
function stringValues(array $shape): void { \PHPStan\dumpType($shape['tail']); }

/** @param array{head?: bool, ...<int>} $shape */
function optionalPrefix(array $shape): void { \PHPStan\dumpType($shape['tail']); }

/** @param array{...<bool>} $shape */
function emptyPrefix(array $shape): void { \PHPStan\dumpType($shape['tail']); }

/** @param non-empty-array{...<string>} $shape */
function nonemptyEmptyPrefix(array $shape): void { \PHPStan\dumpType($shape['tail']); }

/** @param non-empty-array{head?: bool, ...<int>} $shape */
function nonemptyOptionalPrefix(array $shape): void { \PHPStan\dumpType($shape['tail']); }

/** @param array{head: int, ...} $shape */
function untypedExtras(array $shape): void { \PHPStan\dumpType($shape['tail']); }

/** @param array{...} $shape */
function untypedEmptyPrefix(array $shape): void { \PHPStan\dumpType($shape['tail']); }

/** @param array{head?: int, ...<string, bool>} $shape */
function explicitStringKeys(array $shape): void { \PHPStan\dumpType($shape['tail']); }

/** @param array{int, ...<int, string>} $shape */
function explicitIntegerKeys(array $shape): void { \PHPStan\dumpType($shape[123]); }

/** @param array{...<string, bool>} $shape */
function explicitEmptyPrefix(array $shape): void { \PHPStan\dumpType($shape['tail']); }

/** @param list{int, ...<string>} $shape */
function listPrefix(array $shape): void { \PHPStan\dumpType($shape[123]); }

/** @param list{...<bool>} $shape */
function listEmptyPrefix(array $shape): void { \PHPStan\dumpType($shape[123]); }

/** @param non-empty-list{...<int>} $shape */
function nonemptyList(array $shape): void { \PHPStan\dumpType($shape[123]); }

/** @param list{int, ...} $shape */
function untypedList(array $shape): void { \PHPStan\dumpType($shape[123]); }

/** @param non-empty-list{int, ...<bool>} $shape */
function nonemptyListPrefix(array $shape): void { \PHPStan\dumpType($shape[123]); }

/** @param array{head: int, ...<int|string, bool>} $shape */
function explicitUnionKeys(array $shape): void { \PHPStan\dumpType($shape['tail']); }

/** @param array{head?: bool, ...<int|string>} $shape */
function omittedUnionValue(array $shape): void { \PHPStan\dumpType($shape['tail']); }

/**
 * @template T of int|string
 * @param array{head: T, ...<T>} $shape
 */
function templateValues(array $shape): void { \PHPStan\dumpType($shape['tail']); }
