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
 * Represents a file or a URL.
 *
 * @author Nicolas Grekas <p@tchwork.com>
 */
class LinkStub extends ConstStub
{
    public bool $inVendor = false;

    private static array $vendorRoots;
    private static array $composerRoots = [];

    public function __construct(string $label, int $line = 0, ?string $href = null)
    {}

    private function getComposerRoot(string $file, bool &$inVendor): string|false
    {}
}
