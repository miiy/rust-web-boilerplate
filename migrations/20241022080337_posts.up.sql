-- Add up migration script here
START TRANSACTION;

CREATE TABLE IF NOT EXISTS `posts`
(
    `id`                 bigint unsigned NOT NULL AUTO_INCREMENT,
    `category_id`        bigint unsigned NOT NULL DEFAULT 0,
    `title`              varchar(255)    NOT NULL DEFAULT '',
    `author`             varchar(255)    NOT NULL DEFAULT '',
    `content`            text                     DEFAULT NULL,
    `create_time`        timestamp       NULL     DEFAULT NULL,
    `update_time`        timestamp       NULL     DEFAULT NULL,
    `delete_time`        timestamp       NULL     DEFAULT NULL,
    PRIMARY KEY (`id`),
    INDEX `posts_category_id_index` (`category_id`),
    INDEX `posts_create_time_index` (`create_time`)
    ) ENGINE = InnoDB
    DEFAULT CHARSET = utf8mb4
    COLLATE = utf8mb4_unicode_ci;


COMMIT;
