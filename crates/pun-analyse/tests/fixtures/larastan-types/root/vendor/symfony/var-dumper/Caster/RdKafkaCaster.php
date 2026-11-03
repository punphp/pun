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

use RdKafka\Conf;
use RdKafka\Exception as RdKafkaException;
use RdKafka\KafkaConsumer;
use RdKafka\Message;
use RdKafka\Metadata\Broker as BrokerMetadata;
use RdKafka\Metadata\Collection as CollectionMetadata;
use RdKafka\Metadata\Partition as PartitionMetadata;
use RdKafka\Metadata\Topic as TopicMetadata;
use RdKafka\Topic;
use RdKafka\TopicConf;
use RdKafka\TopicPartition;
use Symfony\Component\VarDumper\Cloner\Stub;

/**
 * Casts RdKafka related classes to array representation.
 *
 * @author Romain Neutron <imprec@gmail.com>
 *
 * @internal
 */
class RdKafkaCaster
{
    public static function castKafkaConsumer(KafkaConsumer $c, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castTopic(Topic $c, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castTopicPartition(TopicPartition $c, array $a): array
    {}

    public static function castMessage(Message $c, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castConf(Conf $c, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castTopicConf(TopicConf $c, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castRdKafka(\RdKafka $c, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castCollectionMetadata(CollectionMetadata $c, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castTopicMetadata(TopicMetadata $c, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castPartitionMetadata(PartitionMetadata $c, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castBrokerMetadata(BrokerMetadata $c, array $a, Stub $stub, bool $isNested): array
    {}

    private static function extractMetadata(KafkaConsumer|\RdKafka $c): array
    {}
}
