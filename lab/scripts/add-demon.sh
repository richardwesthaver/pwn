#!/bin/sh
wd=$STASH/tmp
dm=demon

demon-add() {
  useradd $dm -G demon
  mkdir -p /home/$dm
  chown -R $dm:demon /home/$dm
  install -C -m 777 -o $dm -g demon $wd/$dm.tar.zst /home/$dm/
}

demon-del() {
  userdel -f -r $dm
  rm -rf /home/$dm
}
