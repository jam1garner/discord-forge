wget https://raw.githubusercontent.com/ultimate-research/param-labels/master/ParamLabels.csv &&\
	mv ParamLabels.csv param_labels.csv &&\
	echo "Installed param labels"

rm -rf vgaudio/ &&\
	wget https://ci.appveyor.com/api/buildjobs/6v3widme4hdqqwc7/artifacts/VGAudioCli.zip > /dev/null &&\
	unzip VGAudioCli.zip -d vgaudio > /dev/null &&\
	rm VGAudioCli.zip &&\
	echo "Installed VGAudio"

wget https://raw.githubusercontent.com/BenHall-7/msc_labels/master/mscinfo.xml &&\
	cp mscinfo.xml mscdec/ &&\
	mv mscinfo.xml msclang/ &&\
	echo "Installed latest mscinfo.xml"

rm -rf matlab/ &&\
    wget https://github.com/BenHall-7/SSBHLib/releases/download/v1.0/MATLab.zip > /dev/null &&\
    unzip MATLab.zip -d matlab > /dev/null &&\
    rm MATLab.zip &&\
    echo "Installed latest MATLab for .NET Core"

wget https://raw.githubusercontent.com/ultimate-research/param-labels/master/motion_list/Labels.txt &&\
    mv Labels.txt motion_list_labels.txt &&\
    echo "Installed motion_list.bin labels"

wget https://raw.githubusercontent.com/ultimate-research/param-labels/master/sqb/Labels.txt &&\
    mv Labels.txt sqb_labels.txt &&\
    echo "Installed SQB labels"

