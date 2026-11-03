<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Mime\Encoder;

use Symfony\Component\Mime\Exception\RuntimeException;

/**
 * @author Fabien Potencier <fabien@symfony.com>
 */
final class Base64ContentEncoder extends Base64Encoder implements ContentEncoderInterface
{
    public function encodeByteStream($stream, int $maxLineLength = 0): iterable
    { yield; }

    public function getName(): string
    {}
}
