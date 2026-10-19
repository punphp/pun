<?php

namespace TypeAliases;

interface SomeInterface {}
trait SomeTrait {}
enum SomeEnum {}

/**
 * @phpstan-type SomeInterface int
 * @phpstan-type SomeTrait int
 * @phpstan-type SomeEnum int
 * @phpstan-type 123 int
 * @phpstan-import-type 1 from Exporter
 * @phpstan-import-type Exported fromExporter
 * @phpstan-import-type Exported from \TypeAliases\Exporter
 * @phpstan-import-type NoDocAlias from NoDoc
 * @psalm-type Dup string
 * @phpstan-type Dup int
 * @phpstan-type Bad int|
 * @phpstan-type UsesBad Bad
 * @phpstan-type C1 C2|C3
 * @phpstan-type C2 C1
 * @phpstan-type C3 int
 * @phpstan-type Gen Unknown<int>
 * @phpstan-type NotGeneric Exporter<int>
 * @phpstan-type UsesExported Exported
 * @phpstan-type TraitType SomeTrait|SomeTrait
 */
class Duplicates {}

class NoDoc {}

/**
 * @phpstan-type Exported int
 */
class Exporter {}

/**
 * @phpstan-import-type Exported from Exporter as int
 */
class InvalidImportName {}

class Base {}

/**
 * @phpstan-type P parent
 * @phpstan-type S self
 */
class Child extends Base {}

/**
 * @phpstan-import-type X from CircB
 */
class CircA {}

/**
 * @phpstan-import-type X from CircA
 */
class CircB {}

/**
 * @phpstan-import-type X from CircA
 * @phpstan-type L X
 */
class CircUser {}

/**
 * @phpstan-type Me static
 * @phpstan-type Up parent
 * @phpstan-type Missing Nope
 */
trait UsedTrait {}

class UsesTrait extends Base
{
    use UsedTrait;
}

/**
 * @phpstan-type IfaceAlias Unknown
 */
interface AliasInterface {}

/**
 * @phpstan-type EnumAlias Unknown
 */
enum AliasEnum {}
