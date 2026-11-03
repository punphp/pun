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

use Symfony\Component\Mime\Exception\RuntimeException;
use Symfony\Component\Mime\Part\DataPart;
use Symfony\Component\Mime\Part\Multipart\AlternativePart;
use Symfony\Component\Mime\Part\Multipart\MixedPart;
use Symfony\Component\Mime\Part\Multipart\RelatedPart;
use Symfony\Component\Mime\Part\TextPart;

/**
 * @author Fabien Potencier <fabien@symfony.com>
 */
final class MessageConverter
{
    /**
     * @throws RuntimeException when unable to convert the message to an email
     */
    public static function toEmail(Message $message): Email
    {}

    private static function createEmailFromTextPart(Message $message, TextPart $part): Email
    {}

    private static function createEmailFromAlternativePart(Message $message, AlternativePart $part): Email
    {}

    private static function createEmailFromRelatedPart(Message $message, RelatedPart $part): Email
    {}

    private static function addParts(Email $email, array $parts): Email
    {}
}
