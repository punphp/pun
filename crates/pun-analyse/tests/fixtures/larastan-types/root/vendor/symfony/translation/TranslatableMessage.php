<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Translation;

use Symfony\Contracts\Translation\TranslatableInterface;
use Symfony\Contracts\Translation\TranslatorInterface;

/**
 * @author Nate Wiebe <nate@northern.co>
 */
class TranslatableMessage implements TranslatableInterface
{
    public function __construct(
        private string $message,
        private array $parameters = [],
        private ?string $domain = null,
    ) {}

    public function getMessage(): string
    {}

    public function getParameters(): array
    {}

    public function getDomain(): ?string
    {}

    public function trans(TranslatorInterface $translator, ?string $locale = null): string
    {}
}
