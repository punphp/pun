<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Contracts\Translation;

use Symfony\Component\Translation\Exception\InvalidArgumentException;

/**
 * A trait to help implement TranslatorInterface and LocaleAwareInterface.
 *
 * @author Fabien Potencier <fabien@symfony.com>
 */
trait TranslatorTrait
{
    private ?string $locale = null;

    /**
     * @return void
     */
    public function setLocale(string $locale)
    {}

    public function getLocale(): string
    {}

    public function trans(?string $id, array $parameters = [], ?string $domain = null, ?string $locale = null): string
    {}

    /**
     * Returns the plural position to use for the given locale and number.
     *
     * The plural rules are derived from code of the Zend Framework (2010-09-25),
     * which is subject to the new BSD license (http://framework.zend.com/license/new-bsd).
     * Copyright (c) 2005-2010 Zend Technologies USA Inc. (http://www.zend.com)
     */
    private function getPluralizationRule(float $number, string $locale): int
    {}
}
