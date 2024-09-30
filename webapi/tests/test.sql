INSERT INTO
    technologies (id, name, url_name, image_url)
VALUES (
        1,
        'AWS',
        'aws',
        -- awsのロゴ画像のURL
        'https://upload.wikimedia.org/wikipedia/commons/9/93/Amazon_Web_Services_Logo.svg'
    );

INSERT INTO
    projects (id, name, url_name)
VALUES (1, 'ProjectA', 'project-a');

INSERT INTO
    adopts (
        id,
        technologies_id,
        projects_id,
        created_at
    )
VALUES (
        1,
        1,
        1,
        '1970-01-01 00:00:01.000000'
    );