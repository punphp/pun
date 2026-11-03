<?php declare(strict_types = 1);
/*
 * This file is part of PharIo\Manifest.
 *
 * Copyright (c) Arne Blankerts <arne@blankerts.de>, Sebastian Heuer <sebastian@phpeople.de>, Sebastian Bergmann <sebastian@phpunit.de> and contributors
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 *
 */
namespace PharIo\Manifest;

use Iterator;
use function count;

/** @template-implements Iterator<int,Requirement> */
class RequirementCollectionIterator implements Iterator {
    /** @var Requirement[] */
    private $requirements;

    /** @var int */
    private $position = 0;

    public function __construct(RequirementCollection $requirements) {}

    public function rewind(): void {}

    public function valid(): bool {}

    public function key(): int {}

    public function current(): Requirement {}

    public function next(): void {}
}
