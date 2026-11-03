<?php

/**
 * League.Uri (https://uri.thephpleague.com)
 *
 * (c) Ignace Nyamagana Butera <nyamsprod@gmail.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

declare(strict_types=1);

namespace League\Uri;

use finfo;
use League\Uri\Exceptions\MissingFeature;
use League\Uri\IPv4\Calculator;

use function class_exists;
use function defined;
use function extension_loaded;
use function function_exists;

use const PHP_INT_SIZE;

/**
 * Allow detecting features needed to make the packages work.
 */
final class FeatureDetection
{
    public static function supportsFileDetection(): void
    {}

    public static function supportsIdn(): void
    {}

    public static function supportsIPv4Conversion(): void
    {}

    public static function supportsDom(): void
    {}
}
