<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Mime;

use Symfony\Component\Mime\Exception\InvalidArgumentException;
use Symfony\Component\Mime\Exception\LogicException;
use Symfony\Component\Mime\Exception\RuntimeException;

/**
 * Guesses the MIME type using the PECL extension FileInfo.
 *
 * @author Bernhard Schussek <bschussek@gmail.com>
 */
class FileinfoMimeTypeGuesser implements MimeTypeGuesserInterface
{
    /**
     * @var array<string, \finfo>
     */
    private static $finfoCache = [];

    /**
     * @param string|null $magicFile A magic file to use with the finfo instance
     *
     * @see https://php.net/finfo-open
     */
    public function __construct(
        private ?string $magicFile = null,
    ) {}

    public function isGuesserSupported(): bool
    {}

    public function guessMimeType(string $path): ?string
    {}
}
