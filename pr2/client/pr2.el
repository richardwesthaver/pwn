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
(defgroup pr2 nil
  "Poor Richard's Pet Rat")

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
	(insert string))))

(defun pr2-filter (proc str))

(defun pr2-start nil
  "start pr2 client"
  (interactive)
  (unless (process-status "pr2")
    (make-process
     :name "pr2"
     :command '("pr2-client")
     :filter 'pr2-filter
     :sentinel 'pr2-sentinel)))

(provide 'pr2)
;;; pr2.el ends here
