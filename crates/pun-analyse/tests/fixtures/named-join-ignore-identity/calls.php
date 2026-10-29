<?php
class BaseJoinAudit {
    public function run(string $old): void { echo $old; }
}
class ChildJoinAudit extends BaseJoinAudit {
    public function run(string $new): void { echo $new; }
}
function joinAudit(BaseJoinAudit $object): void {
    // @phpstan-ignore argument.parameterRenamedInSubtype.extra, argument.parameterRenamedInSubtype
    $object->run(old: 'suffix first');
    // @phpstan-ignore argument.parameterRenamedInSubtype, argument.parameterRenamedInSubtype.extra
    $object->run(old: 'exact first');
    // @phpstan-ignore argument.parameterRenamedInSubtype
    $object->run(old: 'exact only');
    $object->run(old: 'plain positive');
}
