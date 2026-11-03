<?php

declare(strict_types=1);

namespace Doctrine\Inflector;

use Doctrine\Inflector\Rules\Ruleset;

use function array_unshift;

abstract class GenericLanguageInflectorFactory implements LanguageInflectorFactory
{
    /** @var Ruleset[] */
    private $singularRulesets = [];

    /** @var Ruleset[] */
    private $pluralRulesets = [];

    final public function __construct()
    {}

    final public function build(): Inflector
    {}

    final public function withSingularRules(?Ruleset $singularRules, bool $reset = false): LanguageInflectorFactory
    {}

    final public function withPluralRules(?Ruleset $pluralRules, bool $reset = false): LanguageInflectorFactory
    {}

    abstract protected function getSingularRuleset(): Ruleset;

    abstract protected function getPluralRuleset(): Ruleset;
}
