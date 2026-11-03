<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\VarDumper\Caster;

use Symfony\Component\VarDumper\Cloner\Stub;

/**
 * Casts Amqp related classes to array representation.
 *
 * @author Grégoire Pineau <lyrixx@lyrixx.info>
 *
 * @final
 *
 * @internal
 */
class AmqpCaster
{
    private const FLAGS = [
        \AMQP_DURABLE => 'AMQP_DURABLE',
        \AMQP_PASSIVE => 'AMQP_PASSIVE',
        \AMQP_EXCLUSIVE => 'AMQP_EXCLUSIVE',
        \AMQP_AUTODELETE => 'AMQP_AUTODELETE',
        \AMQP_INTERNAL => 'AMQP_INTERNAL',
        \AMQP_NOLOCAL => 'AMQP_NOLOCAL',
        \AMQP_AUTOACK => 'AMQP_AUTOACK',
        \AMQP_IFEMPTY => 'AMQP_IFEMPTY',
        \AMQP_IFUNUSED => 'AMQP_IFUNUSED',
        \AMQP_MANDATORY => 'AMQP_MANDATORY',
        \AMQP_IMMEDIATE => 'AMQP_IMMEDIATE',
        \AMQP_MULTIPLE => 'AMQP_MULTIPLE',
        \AMQP_NOWAIT => 'AMQP_NOWAIT',
        \AMQP_REQUEUE => 'AMQP_REQUEUE',
    ];

    private const EXCHANGE_TYPES = [
        \AMQP_EX_TYPE_DIRECT => 'AMQP_EX_TYPE_DIRECT',
        \AMQP_EX_TYPE_FANOUT => 'AMQP_EX_TYPE_FANOUT',
        \AMQP_EX_TYPE_TOPIC => 'AMQP_EX_TYPE_TOPIC',
        \AMQP_EX_TYPE_HEADERS => 'AMQP_EX_TYPE_HEADERS',
    ];

    public static function castConnection(\AMQPConnection $c, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castChannel(\AMQPChannel $c, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castQueue(\AMQPQueue $c, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castExchange(\AMQPExchange $c, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castEnvelope(\AMQPEnvelope $c, array $a, Stub $stub, bool $isNested, int $filter = 0): array
    {}

    private static function extractFlags(int $flags): ConstStub
    {}
}
