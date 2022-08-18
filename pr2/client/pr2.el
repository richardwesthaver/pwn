;;; pr2.el --- Poor Richard's Pet Rat -*- lexical-binding: t; -*-

;; Copyright (C) 2022  ellis

;; Author: ellis <ellis@rwest.io>
;; Keywords: processes

;; This program is free software; you can redistribute it and/or modify
;; it under the terms of the GNU General Public License as published by
;; the Free Software Foundation, either version 3 of the License, or
;; (at your option) any later version.

;; This program is distributed in the hope that it will be useful,
;; but WITHOUT ANY WARRANTY; without even the implied warranty of
;; MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
;; GNU General Public License for more details.

;; You should have received a copy of the GNU General Public License
;; along with this program.  If not, see <https://www.gnu.org/licenses/>.

;;; Commentary:

;; https://rwest.io/pwn

;;; Code:
(require 'ewoc)

(defgroup pr2 nil
  "Poor Richard's Pet Rat")

(defcustom pr2-client-port t
  "Port for pr2-client to connect to. A value of `t' will
automatically assign a random port in accordance with the
semantics of `make-network-process' (why not nil??)"
  :group 'pr2)

(defcustom pr2-server-addr [127 0 0 1 9053]
  "socket address for pr2-server"
  :group 'pr2)

(defvar pr2-process nil
  "The pr2 client process handle.")

(defun pr2-sentinel (proc msg)
  (when (string= msg "connection broken by remote peer\n")
    (pr2-log (format "process %s has terminated" proc))))

(defun pr2-log (msg)
  "If a *pr2* buffer exists, write MSG to it for logging purposes."
  (if (get-buffer "*pr2*")
      (with-current-buffer "*pr2*"
	(goto-char (point-max))
	(insert (concat msg "\n")))))

(defun pr2-filter (proc str)
  (let (idx)
    (process-send-string proc msg)
    (while (string-match "\n" str)
      (setq idx (1+ idx))
      (pr2-log (substring msg idx)))))

(defun pr2-start nil
  "start the pr2 client."
  (interactive)
  (unless (process-status "pr2")
    (make-network-process :name "pr2"
			  :buffer "*pr2*"
			  :remote pr2-server-addr
			  :type 'datagram
			  :connection-type 'pipe
			  ;; :coding 'utf-8
			  :sentinel 'pr2-sentinel
			  :filter 'pr2-filter
			  ;; :nowait t
			  )
    (message "pr2-client started")))

(defun my-send-cmd (proc str)
  (if (process-get proc 'my-waiting)
      (process-put proc 'my-pending (append (process-get proc 'my-pending) (list str)))
    (process-put proc 'my-waiting t)
    (process-send-string proc str)))

(defun pr2-stop nil
  "stop the pr2 client."
  (interactive)
  (with-current-buffer "*pr2*"
    (let ((proc (get-buffer-process (current-buffer))))
      (if proc (delete-process proc)))
    (set-buffer-modified-p nil)
    (kill-this-buffer))
  (message "pr2-client stopped"))

(provide 'pr2)
;;; pr2.el ends here
