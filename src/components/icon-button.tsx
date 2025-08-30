import clsx from 'clsx';
import { ButtonProps, buttonVariants } from 'src/components/button';

interface IconButtonProps extends ButtonProps {}

export const IconButton = ({
  children,
  variant = 'primary',
  size = 'md',
  className,
  disabled,
  loading = false,
  ...props
}: IconButtonProps) => {
  return (
    <button
      disabled={disabled || loading}
      className={clsx(
        'rounded-full focus:outline-none flex items-center justify-center',
        buttonVariants.colorVariants[variant],
        buttonVariants.sizeVariants[size],
        (disabled || loading) && 'opacity-50 cursor-not-allowed',
        className
      )}
      {...props}
    >
      {children}
    </button>
  );
};
