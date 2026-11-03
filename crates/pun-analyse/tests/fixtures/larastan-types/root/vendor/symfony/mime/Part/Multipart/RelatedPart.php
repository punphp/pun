<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Mime\Part\Multipart;

use Symfony\Component\Mime\Part\AbstractMultipartPart;
use Symfony\Component\Mime\Part\AbstractPart;

/**
 * @author Fabien Potencier <fabien@symfony.com>
 */
final class RelatedPart extends AbstractMultipartPart
{
    public function __construct(
        private AbstractPart $mainPart,
        AbstractPart $part,
        AbstractPart ...$parts,
    ) {}

    public function getParts(): array
    {}

    public function getMediaSubtype(): string
    {}

    public function __serialize(): array
    {}

    public function __unserialize(array $data): void
    {}

    private function generateContentId(): string
    {}

    private function prepareParts(AbstractPart ...$parts): void
    {}
}
