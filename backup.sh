#!/bin/bash

DT=$(date +%Y-%m-%d-%H-%M)
DIR=".bak"
FILE="${DIR}/${DT}.tgz"
FILE_EXCLUDE="exclude.tag"
mkdir ${DIR} -p
touch .bak/${FILE_EXCLUDE}
touch volumes/${FILE_EXCLUDE}
touch target/${FILE_EXCLUDE}
touch documents/${FILE_EXCLUDE}
touch documents_not_used/${FILE_EXCLUDE}

tar -zcvf ${FILE} \
	--exclude-tag-all=${FILE_EXCLUDE} \
	--exclude='FILE|DIR' \
	.