rm -rf paramxml/
wget https://github.com/BenHall-7/paracobNET/releases/download/1.99/Release.zip > /dev/null
unzip Release.zip -d paramxml > /dev/null
rm Release.zip
echo "Installed ParamXML 1.99"
wget https://raw.githubusercontent.com/ultimate-research/param-labels/master/ParamLabels.csv
mv ParamLabels.csv paramxml/netcoreapp2.1/
echo "Installed latest ParamLabels.csv"

