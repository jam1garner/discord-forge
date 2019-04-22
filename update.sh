rm -rf paramxml/
wget https://github.com/BenHall-7/paracobNET/releases/download/1.99/Release.zip > /dev/null
unzip Release.zip -d paramxml > /dev/null
rm Release.zip
echo "Installed ParamXML 1.99"

wget https://raw.githubusercontent.com/ultimate-research/param-labels/master/ParamLabels.csv
mv ParamLabels.csv paramxml/netcoreapp2.1/
echo "Installed latest ParamLabels.csv"

rm -rf vgaudio/
wget https://ci.appveyor.com/api/buildjobs/wa2ie68wd0eq51sw/artifacts/VGAudioCli.zip > /dev/null
unzip VGAudioCli.zip -d vgaudio > /dev/null
rm VGAudioCli.zip
echo "Installed VGAudio"
