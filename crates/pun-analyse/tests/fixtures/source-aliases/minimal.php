<?php
#[Attribute] class Original {}
class_alias(Original::class, 'AliasName');
#[AliasName] class Applied {}
new AliasName();
