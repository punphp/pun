<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\VarDumper\Caster;

/**
 * Represents a cut array.
 *
 * @author Nicolas Grekas <p@tchwork.com>
 */
class CutArrayStub extends CutStub
{
    public array $preservedSubset;

    public function __construct(array $value, array $preservedKeys)
    {}
}
