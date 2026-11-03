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

use Countable;
use IteratorAggregate;
use function count;

/** @template-implements IteratorAggregate<int,Requirement> */
class RequirementCollection implements Countable, IteratorAggregate {
    /** @var Requirement[] */
    private $requirements = [];

    public function add(Requirement $requirement): void {}

    /**
     * @return Requirement[]
     */
    public function getRequirements(): array {}

    public function count(): int {}

    public function getIterator(): RequirementCollectionIterator {}
}
