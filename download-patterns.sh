echo "Downloading patterns from LifeWiki..."
URL="http://www.conwaylife.com/patterns/all.zip"
OUTPUT_FILE="patterns.zip"

WGET=`which wget`
CURL=`which curl`

mkdir -p patterns/

if [ ! -z $WGET ]
then
    $WGET $URL -O $OUTPUT_FILE
else
    if [ ! -z $CURL ]
    then
        $CURL $URL -o $OUTPUT_FILE
    else
        echo "Error downloading patterns"
    fi
fi

echo "Unpacking patterns..."
unzip -o $OUTPUT_FILE -d patterns/

echo "Cleaning up..."
find patterns/ -type f ! -name *.rle -delete
rm patterns.zip
touch patterns/.directory

echo "done."
