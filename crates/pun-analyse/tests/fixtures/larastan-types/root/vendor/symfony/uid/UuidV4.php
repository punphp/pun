<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Uid;

/**
 * A v4 UUID contains a 122-bit random number.
 *
 * @author Grégoire Pineau <lyrixx@lyrixx.info>
 */
class UuidV4 extends Uuid
{
    protected const TYPE = 4;

    public function __construct(?string $uuid = null)
    {}
}
