<?php

namespace VariableNameTemplates;

/**
 * @template Name of 'missingLiteral'
 * @param Name $name
 */
function literalName($name): void
{
    compact($name);
    echo $$name;
    $copy = $name;
    compact($copy);
    echo ${$copy};
}

/**
 * @template Name of 'existing'|'missingUnion'
 * @param Name $name
 */
function unionBound($name): void
{
    $existing = 1;
    compact($name);
    echo $$name;
}

/**
 * @template Name of 'first'
 * @param Name|'second' $name
 */
function unionWithTemplate($name): void
{
    compact($name);
    echo $$name;
}

/**
 * @template Names of array{'missingNested', array{'missingDeep', 'existing'}}
 * @param Names $names
 */
function nestedArray($names): void
{
    $existing = 1;
    compact($names);
    $copy = $names;
    compact($copy);
    $copy = ['existing'];
    compact($copy);
}

/**
 * @template Name of 'nestedTemplate'
 * @template Names of array{Name, array{Name, 'existing'}}
 * @param Names $names
 */
function templatesInsideArray($names): void
{
    $existing = 1;
    compact($names);
}

/**
 * @template Name of 'inheritedBound'
 * @template Child of Name
 * @param Child $name
 */
function nestedBounds($name): void
{
    compact($name);
    echo $$name;
}

/**
 * @template Names of array{'arrayUnionFirst'}|array{'arrayUnionSecond'}
 * @param Names $names
 */
function arrayUnionBound($names): void
{
    compact($names);
}

/**
 * @template Name of 'maybe'
 * @param Name $name
 */
function possibleName($name, bool $flag): void
{
    if ($flag) {
        $maybe = 1;
    }
    compact($name);
    echo $$name;
}

/**
 * @template Name of string
 * @template Names of array<string>
 * @param Name $name
 * @param Names $names
 */
function generalBounds($name, $names): void
{
    compact($name, $names);
    echo $$name;
}

/**
 * @template Name of 'notCertain'|int
 * @template Names of array{'notCertainArray'}|array<string>
 * @param Name $name
 * @param Names $names
 */
function nonConstantUnionControls($name, $names): void
{
    compact($name, $names);
    echo $$name;
}

/**
 * @template Name of 'overwritten'
 * @param Name $name
 */
function overwrittenCopy($name): void
{
    $copy = $name;
    $copy = 'existing';
    $existing = 1;
    compact($copy);
    echo $$copy;
}

/**
 * @template Names of array{'arrayParent', array{'arrayChild'}}
 * @template Child of Names
 * @param Child $names
 */
function nestedArrayBound($names): void
{
    compact($names);
}

/**
 * @template Name of 'unionParent'|'unionChild'
 * @template Child of Name
 * @param Child $name
 */
function nestedUnionBound($name): void
{
    compact($name);
    echo $$name;
}

/**
 * @template Left of 'overlap'|'left'
 * @template Right of 'overlap'|'right'
 * @param Left|Right $name
 */
function overlappingTemplateBounds($name): void
{
    compact($name);
    echo $$name;
}
